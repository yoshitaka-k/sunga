use std::sync::Arc;
use std::sync::mpsc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::file;

const RUNNING_COUNT_NUM: usize = 1;
const CONCURRENT_NUM: u8 = 10;

/// 寸法読み込みジョブ
pub(crate) struct DimensionsJob {
    /// コンテキスト
    ctx: egui::Context,

    /// ジョブ結果を送信するチャネル
    result_tx: mpsc::Sender<file::Image>,
    /// ジョブ結果を受信するチャネル
    result_rx: mpsc::Receiver<file::Image>,

    /// ジョブ実行中のカウント
    running_count: Arc<AtomicUsize>,
}

impl DimensionsJob {
    /// 新しい寸法読み込みジョブを作成
    /// * `ctx` - コンテキスト
    /// * `return` - 寸法読み込みジョブ
    pub fn new(ctx: egui::Context) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self {
            ctx,
            result_tx,
            result_rx,
            running_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// ジョブを実行
    /// * `files` - 開いているファイル
    pub fn run(&self, files: &mut file::OpenFiles) {
        // ジョブ実行可能な間、ジョブを実行
        while self.is_runnable() {
            // 寸法を読み込んでないファイルがない場合は、ジョブを終了
            let Some(image) = files.get_pending_file() else { break };

            // 寸法を読み込み中に設定
            image.set_dimensions(file::Dimensions::Loading);

            // 必要なものをクローン
            let mut image = image.clone();
            let ctx = self.ctx.clone();
            let result_tx = self.result_tx.clone();

            // ジョブ実行中のカウントを増加
            self.running_count.fetch_add(RUNNING_COUNT_NUM, Ordering::Relaxed);
            let running_count = Arc::clone(&self.running_count);

            // ジョブ実行スレッドを作成
            std::thread::spawn(move || {
                // ファイルの寸法を取得
                if let Some(dimensions) = file::image::open_dimensions(image.path()) {
                    let (width, height) = dimensions;
                    image.set_dimensions(file::Dimensions::Ready(width, height));
                } else {
                    image.set_dimensions(file::Dimensions::Unavailable);
                }

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
            // ジョブ結果を反映
            files.apply_dimensions_result(result);
        }
    }
}

/// private methods
impl DimensionsJob {
    /// ジョブ実行可能かどうか
    /// * `return` - ジョブ実行可能かどうか
    fn is_runnable(&self) -> bool {
        self.running_count.load(Ordering::Relaxed) < CONCURRENT_NUM as usize
    }
}
