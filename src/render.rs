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
        cc.egui_ctx.set_visuals(config.theme().clone());

        // 範囲外の値をクランプ
        let scale = config.scale_mut();
        *scale = (*scale).clamp(resize::SCALE_RANGE_MIN, resize::SCALE_RANGE_MAX);
        let width = config.width_mut();
        *width = (*width).clamp(resize::SIZED_RANGE_MIN, resize::SIZED_RANGE_MAX);
        let height = config.height_mut();
        *height = (*height).clamp(resize::SIZED_RANGE_MIN, resize::SIZED_RANGE_MAX);

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
            resize_job: resize::ResizeJob::new(cc.egui_ctx.clone()),
            update_job: update::UpdateJob::new(cc.egui_ctx.clone()),
        };

        Self {
            state,
            jobs,
            pending_actions: Vec::new(),
            sound: sounds::SoundPlayer::new(),
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

        // リサイズジョブを実行
        self.jobs.resize_job.run(&self.state.config, &mut self.state.files);

        // ファイル情報を更新
        self.state.files.update_file_info();

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
        // 待機中かリサイズ中かどうかをチェック
        let was_busy = self.state.files.has_standby() || self.state.files.has_resizing();

        // リサイズ結果を受信
        self.jobs.resize_job.result(&mut self.state.files);

        // 最適化後か最適化中が終わっているかどうかをチェック
        let now_idle = !self.state.files.has_standby() && !self.state.files.has_resizing();

        // 効果音を再生
        if *self.state.config.play_sound() && was_busy && now_idle {
            // 効果音の音量を設定
            self.sound.set_volume(*self.state.config.sound_volume());

            if self.state.files.has_error() {
                self.sound.play_alert();
            } else {
                self.sound.play_completed();
            }
        }

        // 更新結果を取得
        self.jobs.update_job.result(&mut self.state.updated_token);
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
                log::error!("Error processing action: {}", e);
                self.state.error_token.reset();
                self.state.error_token.show(e);
            }

            // 再描画を要求
            ui.ctx().request_repaint();
        }
    }
}
