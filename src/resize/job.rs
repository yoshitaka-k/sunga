use std::sync::Arc;
use std::sync::{mpsc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::collections::HashSet;
use crate::{app, error, file};

// リサイズ実行中のカウント数
const RUNNING_COUNT_NUM: usize = 1;

/// リサイズジョブ
pub struct ResizeJob {
    /// コンテキスト
    ctx: egui::Context,

    /// リサイズ結果を送信するチャネル
    result_tx: mpsc::Sender<file::Image>,
    /// リサイズ結果を受信するチャネル
    result_rx: mpsc::Receiver<file::Image>,

    /// リサイズ実行フラグ
    running: Arc<AtomicBool>,

    /// リサイズ実行中のカウント
    running_count: Arc<AtomicUsize>,

    /// キャンセルフラグ
    canceled: Arc<Mutex<HashSet<u64>>>,
}

impl ResizeJob {
    /// 新しいリサイズジョブを作成
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

    /// リサイズジョブを実行
    /// * `files` - 開いているファイル
    /// * `return` - リサイズジョブ
    pub fn run(&self, app: &app::App, files: &mut file::OpenFiles) {
        // ファイルがない場合は、リサイズを行わない
        if files.is_empty() { return; }

        // 待機中のファイルがあったら、リサイズを開始
        if files.has_standby() {
            self.running.store(true, Ordering::Relaxed);
        }

        // 最適化実行可能なら、リサイズを続ける
        while self.is_runnable(app) {
            // 待機中のファイルがない場合は、リサイズを終了
            let Some(file) = files.get_standby_file() else { break };

            // リサイズステータスを設定して、クローン
            file.set_status(file::Status::Resizing);
            let mut file = file.clone();

            // そのほかもクローン
            let app = app.clone();
            let ctx = self.ctx.clone();
            let result_tx = self.result_tx.clone();
            let running = self.running.clone();
            let canceled = self.canceled.clone();

            // リサイズ実行中のカウントを追加してクローン
            self.running_count.fetch_add(RUNNING_COUNT_NUM, Ordering::Relaxed);
            let running_count = self.clone_running_count();

            // リサイズの実行スレッドを作成
            std::thread::spawn(move || {
                // リサイズを実行
                let _ = file.resize(&app, Arc::clone(&running), Arc::clone(&canceled));

                // リサイズ結果を送信
                let _ = result_tx.send(file);

                // リサイズ実行中のカウントを減少
                running_count.fetch_sub(RUNNING_COUNT_NUM, Ordering::Relaxed);

                // 再描画を要求
                ctx.request_repaint();
            });
        }
    }

    /// リサイズ結果を受信
    pub fn result(&self, files: &mut file::OpenFiles) {
        // リサイズ結果を受信
        while let Ok(result) = self.result_rx.try_recv() {
            // ファイル ID を控えておく
            let id = *result.id();

            // リサイズ結果を反映
            files.apply_result(result);

            // キャンセルされたファイル ID を削除
            let _ = self.remove_canceled_id(id);
        }
    }

    /// リサイズをキャンセル
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
    /// リサイズ実行可能かどうか
    /// * `app` - アプリケーション
    /// * `return` - リサイズ実行可能かどうか
    fn is_runnable(&self, app: &app::App) -> bool {
        self.running_count.load(Ordering::Relaxed) < *app.concurrent_num() as usize
    }

    /// リサイズ実行中のカウントをクローン
    /// * `return` - リサイズ実行中のカウント
    fn clone_running_count(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.running_count)
    }

    /// キャンセルされたファイル ID を削除
    /// * `id` - ファイル ID
    /// * `return` - キャンセルされたファイル ID を削除
    fn remove_canceled_id(&self, id: u64) -> error::Result<()> {
        self.canceled.lock().map_err(|_| {
            error::SungaError::lock_poisoned()
        })?.remove(&id);
        Ok(())
    }
}
