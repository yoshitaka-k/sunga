use std::sync::mpsc;
use crate::REQUEST_URL;
use crate::update;

/// 更新ジョブを管理する構造体
pub(crate) struct UpdateJob {
    ctx: egui::Context,

    /// 更新結果を送信するチャネル
    result_tx: mpsc::Sender<update::UpdatedToken>,
    /// 更新結果を受信するチャネル
    result_rx: mpsc::Receiver<update::UpdatedToken>,

    /// 更新モーダルを表示するためのトークン
    updated_token: update::UpdatedToken,
}

/// public methods
impl UpdateJob {
    /// 新しい更新ジョブを作成する
    /// * `ctx` - コンテキスト
    /// * `return` - 更新ジョブ
    pub fn new(ctx: egui::Context) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self {
            ctx,
            result_tx,
            result_rx,
            updated_token: update::UpdatedToken {
                open: false,
                check: None,
            },
        }
    }

    /// 更新を実行する
    pub fn run(&self) {
        // クローンしておく
        let tx = self.result_tx.clone();
        let ctx = self.ctx.clone();
        let mut token = self.updated_token.clone();

        let request_url = REQUEST_URL.replace("{repository}", env!("CARGO_PKG_REPOSITORY"));
        let request = ehttp::Request::get(request_url);

        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            match result {
                // リクエストが成功した場合
                Ok(response) => {
                    // アップデート情報をJSONからパース
                    let update_info: Result<update::UpdateInfo, _> = response.json();
                    match update_info {
                        // アップデート情報が取得できた場合
                        Ok(update_info) => {
                            match update::compare_version(update_info.tag_name(), env!("CARGO_PKG_VERSION")) {
                                Ok(true) => {
                                    token.open = true;
                                    token.check = Some(update::UpdateCheck::Available {
                                        version: update_info.tag_name().clone(),
                                        url: update_info.html_url.clone(),
                                    });
                                }
                                Ok(false) => {
                                    token.open = true;
                                    token.check = Some(update::UpdateCheck::Latest);
                                }
                                Err(error) => {
                                    eprintln!("error: {:?}", error);
                                    token.open = true;
                                    token.check = Some(update::UpdateCheck::Failed(error.to_string()));
                                }
                            }
                        }
                        // アップデート情報が取得できなかった場合
                        Err(error) => {
                            eprintln!("error: {:?}", error);
                            token.open = true;
                            token.check = Some(update::UpdateCheck::Failed(error.to_string()));
                        }
                    }
                }
                // リクエストが失敗した場合
                Err(error) => {
                    eprintln!("error: {:?}", error);
                    token.open = true;
                    token.check = Some(update::UpdateCheck::Failed(error.to_string()));
                }
            }

            // 更新モーダルを表示するためのトークンを送信
            let _ = tx.send(token.clone());

            // 再描画を要求
            ctx.request_repaint();
        });
    }

    /// 更新結果を受信する
    /// * `updated_token` - 更新結果を受信するトークン
    pub fn result(&self, updated_token: &mut update::UpdatedToken) {
        while let Ok(result) = self.result_rx.try_recv() {
            *updated_token = result;
        }
    }
}
