use std::sync::Arc;
use std::sync::{mpsc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::collections::HashSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::{config, error, file, resize};

// ジョブ実行中のカウント数
const RUNNING_COUNT_NUM: usize = 1;

/// リサイズジョブ
pub struct ResizeJob {
    /// コンテキスト
    ctx: egui::Context,

    /// ジョブ結果を送信するチャネル
    result_tx: mpsc::Sender<file::Image>,
    /// ジョブ結果を受信するチャネル
    result_rx: mpsc::Receiver<file::Image>,

    /// ジョブ実行フラグ
    running: Arc<AtomicBool>,

    /// ジョブ実行中のカウント
    running_count: Arc<AtomicUsize>,

    /// キャンセルフラグ
    canceled: Arc<Mutex<HashSet<u64>>>,
}

impl ResizeJob {
    /// 新しいジョブを作成
    /// * `ctx` - コンテキスト
    /// * `return` - リサイズジョブ
    pub fn new(ctx: egui::Context) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self {
            ctx,
            result_tx,
            result_rx,
            running: Arc::new(AtomicBool::new(false)),
            running_count: Arc::new(AtomicUsize::new(0)),
            canceled: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// ジョブを実行
    /// * `config` - 設定
    /// * `files` - 開いているファイル
    pub fn run(&self, config: &config::Config, files: &mut file::OpenFiles) {
        // ファイルがない場合は、ジョブを行わない
        if files.is_empty() { return; }

        // リサイズオプション
        let options = resize::Options {
            resize_policy: *config.resize_policy(),
            target: config.resize_target(),
            filter: *config.filter(),
        };

        // 設定されている値が範囲外の場合はエラーメッセージを表示して終了
        if let Some(msg) = options.target.validate() {
            while let Some(image) = files.get_standby_file() {
                image.set_status(file::Status::Error(msg.clone()));
            }
            return;
        }

        // 待機中のファイルがあったら、ジョブを開始
        if files.has_standby() {
            self.running.store(true, Ordering::Relaxed);
        }

        // 最適化実行可能なら、ジョブを続ける
        while self.is_runnable(config) {
            // 待機中のファイルがない場合は、ジョブを終了
            let Some(image) = files.get_standby_file() else { break };

            // ジョブステータスを設定して、クローン
            image.set_status(file::Status::Resizing);
            let mut image = image.clone();

            // そのほかもクローン
            let options = options.clone();
            let ctx = self.ctx.clone();
            let result_tx = self.result_tx.clone();
            let running = Arc::clone(&self.running);
            let canceled = Arc::clone(&self.canceled);

            // ジョブ実行中のカウントを追加してクローン
            self.running_count.fetch_add(RUNNING_COUNT_NUM, Ordering::Relaxed);
            let running_count = self.clone_running_count();

            // ジョブの実行スレッドを作成
            std::thread::spawn(move || {
                // ジョブ中のパニックをキャッチ
                let result = catch_unwind(AssertUnwindSafe(|| {
                    // ジョブを実行
                    Self::resize(
                        &mut image,
                        &options,
                        running,
                        canceled,
                    )
                }));

                // ジョブ結果からステータスを取得
                let status = match result {
                    Ok(Ok(status)) => status,
                    Ok(Err(e)) => file::Status::Error(e.to_string()),
                    Err(payload) => {
                        let message = payload.downcast_ref::<&str>().map(|s| s.to_string())
                            .or_else(|| payload.downcast_ref::<String>().cloned())
                            .unwrap_or_else(|| "Unknown error".to_string());
                        file::Status::Error(message)
                    }
                };

                // ステータスを設定
                image.set_status(status);

                // ジョブ結果を送信
                let _ = result_tx.send(image);

                // ジョブ実行中のカウントを減少
                running_count.fetch_sub(RUNNING_COUNT_NUM, Ordering::Relaxed);

                // 再描画を要求
                ctx.request_repaint();
            });
        }
    }

    /// ジョブ結果を受信
    pub fn result(&self, files: &mut file::OpenFiles) {
        // ジョブ結果を受信
        while let Ok(result) = self.result_rx.try_recv() {
            // ファイル ID を控えておく
            let id = *result.id();

            // ジョブ結果を反映
            files.apply_result(result);

            // キャンセルされたファイル ID を削除
            let _ = self.remove_canceled_id(id);
        }
    }

    /// ジョブをキャンセル
    pub fn stop_running(&self) {
        self.running.store(false, Ordering::Release);
    }

    /// キャンセルに登録されているファイル ID を追加
    /// * `id` - ファイル ID
    /// * `return` - 成功かどうか
    pub fn add_canceled_id(&self, id: u64) -> error::Result<()> {
        self.canceled.lock().map_err(|_| error::SungaError::lock_poisoned())?.insert(id);
        Ok(())
    }
}

/// private methods
impl ResizeJob {
    /// ジョブ実行可能かどうか
    /// * `config` - 設定
    /// * `return` - ジョブ実行可能かどうか
    fn is_runnable(&self, config: &config::Config) -> bool {
        self.running_count.load(Ordering::Relaxed) < *config.concurrent_num() as usize
    }

    /// ジョブ実行中のカウントをクローン
    /// * `return` - ジョブ実行中のカウント
    fn clone_running_count(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.running_count)
    }

    /// キャンセルされたファイル ID を削除
    /// * `id` - ファイル ID
    /// * `return` - キャンセルされたファイル ID を削除したかどうか
    fn remove_canceled_id(&self, id: u64) -> error::Result<()> {
        self.canceled.lock().map_err(|_| {
            error::SungaError::lock_poisoned()
        })?.remove(&id);
        Ok(())
    }

    /// 画像をリサイズ
    /// * `image` - リサイズする画像
    /// * `options` - リサイズオプション
    /// * `running` - ジョブ実行フラグ
    /// * `canceled` - キャンセルフラグ
    /// * `return` - リサイズステータス
    fn resize(
        image: &mut file::Image,
        options: &resize::Options,
        running: Arc<AtomicBool>,
        canceled: Arc<Mutex<HashSet<u64>>>,
    ) -> error::Result<file::Status> {
        // 待機中・リサイズ中状態以外は実行しない
        if !image.is_standby_or_resizing() {
            return Ok(image.status().clone());
        }

        // 処理開始時間を取得
        let start_time = std::time::Instant::now();

        // リサイズトークン
        let token = resize::Token {
            id: *image.id(),
            running,
            canceled,
        };

        // 出力先を取得
        let Some(output_path) = image.output_path() else {
            return Ok(file::Status::Error("Output path is not set".to_string()));
        };

        // リサイズを実行してステータスを取得
        let status = match resize::Resize::execute(image.path(), output_path, options, &token)? {
            resize::ExecuteResult::Success { width, height } => {
                // リサイズ結果を設定
                image.set_resized((width, height), start_time.elapsed().as_millis() as u64);
                file::Status::Resized
            }
            resize::ExecuteResult::Canceled => file::Status::Canceled,
            resize::ExecuteResult::Unchanged => file::Status::Unchanged,
        };

        Ok(status)
    }
}
