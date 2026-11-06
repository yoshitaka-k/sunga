use crate::{config, lang, event, file, resize, update, state, sounds, ui};
use crate::event::{input, handler};

/// アプリケーション本体
pub struct Render {
    /// アプリケーションの状態
    state: state::State,

    /// ジョブ
    jobs: state::Jobs,

    /// 待機中のアクション
    pending_actions: Vec<event::EventAction>,

    /// 効果音プレイヤー
    sound: sounds::SoundPlayer,

    /// スキャン中か待機中かリサイズ中かどうか
    was_busy: bool,
}

impl Render {
    /// 新しい Render を作成
    /// * `cc` - 作成コンテキスト
    /// * `config` - 設定
    /// * `return` - 新しい Render
    pub fn new(cc: &eframe::CreationContext<'_>, config: config::Config) -> Self {
        // アセットを追加
        ui::install(&cc.egui_ctx);

        // スタイルを設定
        cc.egui_ctx.global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // 前回保存した Config があれば復元（なければ引数の config を使う）
        let mut config = cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or(config);

        // 言語を設定
        lang::set(config.language());

        // テーマを設定
        let theme = match config.theme() {
            ui::Theme::Dark => egui::Visuals::dark(),
            ui::Theme::Light => egui::Visuals::light(),
        };
        cc.egui_ctx.set_visuals(theme);

        // 範囲外の値をクランプ
        let scale = config.scale_mut();
        *scale = (*scale).clamp(resize::SCALE_RANGE_MIN, resize::SCALE_RANGE_MAX);
        let width = config.width_mut();
        *width = (*width).clamp(resize::SIZED_RANGE_MIN, resize::SIZED_RANGE_MAX);
        let height = config.height_mut();
        *height = (*height).clamp(resize::SIZED_RANGE_MIN, resize::SIZED_RANGE_MAX);
        let concurrent_num = config.concurrent_num_mut();
        *concurrent_num = (*concurrent_num).clamp(resize::CONCURRENT_NUM_MIN, resize::CONCURRENT_NUM_MAX);
        let sound_volume = config.sound_volume_mut();
        *sound_volume = (*sound_volume).clamp(config::SOUND_VOLUME_MIN, config::SOUND_VOLUME_MAX);

        // アプリケーションの状態を作成
        let state = state::State {
            config,
            files: file::OpenFiles::new(),
            setting_token: state::SettingToken::new(),
            updated_token: update::UpdatedToken::new(),
            error_token: state::ErrorToken::new(),
            scroll_token: state::ScrollToken::new(),
        };

        // ジョブを作成
        let jobs = state::Jobs {
            scan_job: file::ScanJob::new(cc.egui_ctx.clone()),
            dimensions_job: file::DimensionsJob::new(cc.egui_ctx.clone()),
            resize_job: resize::ResizeJob::new(cc.egui_ctx.clone()),
            update_job: update::UpdateJob::new(cc.egui_ctx.clone()),
        };

        Self {
            state,
            jobs,
            pending_actions: Vec::new(),
            sound: sounds::SoundPlayer::new(),
            was_busy: false,
        }
    }
}

/// eframe::App の実装
impl eframe::App for Render {
    /// 終了前に Config の状態を保存
    /// * `storage` - ストレージ
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state.config);
    }

    /// 描画
    /// * `ui` - UI
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // ジョブの結果を受信
        self.jobs_result();

        // 選択された行を解除はイベント・アクションの前
        self.state.scroll_token.selected = false;

        // 入力イベントを処理
        self.input_events(ui);

        // 待機中のアクションを処理
        self.process_pending_actions(ui);

        // 寸法読み込みジョブを実行
        self.jobs.dimensions_job.run(&mut self.state.files);

        // リサイズジョブを実行
        self.jobs.resize_job.run(&self.state.config, &mut self.state.files);

        // ファイル情報を更新
        self.state.files.update_file_info();

        // 効果音を再生
        self.finished_play_sound();

        // ステータスカラーを作成
        let status_color = ui::StatusColor::new(ui.ctx());

        // 描画は一番最後
        ui::draw(ui, &mut self.state, &mut self.pending_actions, &status_color);
    }
}

/// private methods
impl Render {
    /// 結果を処理
    fn jobs_result(&mut self) {
        // ファイルスキャン結果を受信
        self.jobs.scan_job.result(&self.state.config, &mut self.state.files);

        // 寸法読み込み結果を受信
        self.jobs.dimensions_job.result(&mut self.state.files);

        // リサイズ結果を受信
        self.jobs.resize_job.result(&mut self.state.files);

        // 更新結果を取得
        self.jobs.update_job.result(&mut self.state.updated_token);
    }

    /// 効果音を再生
    fn finished_play_sound(&mut self) {
        // スキャン中か待機中かリサイズ中？（忙しいかどうか）
        let busy = self.is_busy();

        // ファイルがあって、忙しい状態が終わったら効果音を再生
        if !self.state.files.is_empty() && *self.state.config.play_sound() && self.was_busy && !busy {
            // 効果音の音量を設定
            self.sound.set_volume(*self.state.config.sound_volume());

            if self.state.files.has_error() {
                self.sound.play_alert();
            } else {
                self.sound.play_completed();
            }
        }

        // 忙しいかどうかを更新
        self.was_busy = busy;
    }

    /// 入力イベントを処理
    /// * `ui` - UI
    fn input_events(&mut self, ui: &mut egui::Ui) {
        // ドロップされたファイルを処理
        input::drop(ui, &mut self.pending_actions);

        // Command + O キーが押されたらファイルを開く
        input::command_open(ui, &mut self.pending_actions);

        // Command + Comma キーが押されたら設定ウィンドウを開く
        input::command_comma(ui, &mut self.pending_actions);

        // 上矢印キーが押されたら選択された行を上に移動する
        input::up(ui, &self.state.files, &mut self.pending_actions);

        // 下矢印キーが押されたら選択された行を下に移動する
        input::down(ui, &self.state.files, &mut self.pending_actions);

        // エンターキーが押されたら選択された行を開く
        input::enter(ui, &self.state.files, &mut self.pending_actions);

        // スペースキーが押されたら選択された行を開く
        input::space(ui, &self.state.files, &mut self.pending_actions);

        // バックスペースキーが押されたら選択された行をキャンセル
        input::backspace(ui, &mut self.pending_actions);
    }

    /// 待機中のアクションを処理
    /// * `pending_actions` - 待機中のアクション
    fn process_pending_actions(&mut self, ui: &egui::Ui) {
        for action in self.pending_actions.drain(..) {
            if let Err(e) = handler::handle(ui.ctx(), &mut self.state, &mut self.jobs, action) {
                self.state.error_token.reset();
                self.state.error_token.show(e);
            }

            // 再描画を要求
            ui.ctx().request_repaint();
        }
    }

    /// スキャン中か待機中かリサイズ中かどうかをチェック
    /// * `return` - スキャン中か待機中かリサイズ中かどうか
    fn is_busy(&self) -> bool {
        self.jobs.scan_job.is_running()
            || self.state.files.has_standby()
            || self.state.files.has_resizing()
    }
}
