use std::sync::mpsc;
use serde::Deserialize;
use getset::Getters;

use crate::REQUEST_URL;
use crate::error;

/// アップデート確認の結果
#[derive(Clone)]
pub(crate) enum UpdateCheck {
    Available { version: String, url: String },
    Latest,
    Failed(String),
}

/// 更新モーダルを表示するためのトークン
#[derive(Clone)]
pub(crate) struct UpdatedToken {
    pub open: bool,
    pub check: Option<UpdateCheck>,
}
impl UpdatedToken {
    pub fn new() -> Self {
        Self {
            open: false,
            check: None,
        }
    }
}

/// GitHub レスポンスのアップデート情報
#[derive(Deserialize, Getters)]
#[getset(get)]
struct UpdateInfo {
    tag_name: String,
    html_url: String,
}

/// 更新ジョブを管理する構造体
pub(crate) struct UpdateJob {
    ctx: egui::Context,

    /// 更新結果を送信するチャネル
    result_tx: mpsc::Sender<UpdatedToken>,
    /// 更新結果を受信するチャネル
    result_rx: mpsc::Receiver<UpdatedToken>,

    /// 更新モーダルを表示するためのトークン
    updated_token: UpdatedToken,
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
            updated_token: UpdatedToken {
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
                    let update_info: Result<UpdateInfo, _> = response.json();
                    match update_info {
                        // アップデート情報が取得できた場合
                        Ok(update_info) => {
                            match compare_version(update_info.tag_name(), env!("CARGO_PKG_VERSION")) {
                                Ok(true) => {
                                    token.open = true;
                                    token.check = Some(UpdateCheck::Available {
                                        version: update_info.tag_name().clone(),
                                        url: update_info.html_url.clone(),
                                    });
                                }
                                Ok(false) => {
                                    token.open = true;
                                    token.check = Some(UpdateCheck::Latest);
                                }
                                Err(error) => {
                                    println!("error: {:?}", error);
                                    token.open = true;
                                    token.check = Some(UpdateCheck::Failed(error.to_string()));
                                }
                            }
                        }
                        // アップデート情報が取得できなかった場合
                        Err(error) => {
                            println!("error: {:?}", error);
                            token.open = true;
                            token.check = Some(UpdateCheck::Failed(error.to_string()));
                        }
                    }
                }
                // リクエストが失敗した場合
                Err(error) => {
                    println!("error: {:?}", error);
                    token.open = true;
                    token.check = Some(UpdateCheck::Failed(error.to_string()));
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
    pub fn result(&self, updated_token: &mut UpdatedToken) {
        while let Ok(result) = self.result_rx.try_recv() {
            *updated_token = result;
        }
    }
}

/// バージョンを比較する
/// * `new` - 新しいバージョン
/// * `old` - 古いバージョン
/// * `return` - 比較結果
fn compare_version(new: &str, old: &str) -> error::Result<bool> {
    let new = new.replace("v", "");
    let old = old.replace("v", "");

    // 新しいバージョンをメジャー、マイナー、パッチに分割
    let parts: Vec<i32> = new.split(".")
        .map(|x| x.parse::<i32>())
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| error::SungaError::invalid_version())?;
    let [new_major, new_minor, new_patch]: [i32; 3] = parts.try_into()
        .map_err(|_| error::SungaError::invalid_version())?;

    // 古いバージョンをメジャー、マイナー、パッチに分割
    let parts: Vec<i32> = old.split(".")
        .map(|x| x.parse::<i32>())
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| error::SungaError::invalid_version())?;
    let [old_major, old_minor, old_patch]: [i32; 3] = parts.try_into()
        .map_err(|_| error::SungaError::invalid_version())?;

    // メジャー、マイナー、パッチを比較
    if new_major != old_major {
        // メジャー番号が異なる場合は、メジャー番号の比較
        Ok(new_major > old_major)
    } else if new_minor != old_minor {
        // マイナー番号が異なる場合は、マイナー番号を比較
        Ok(new_minor > old_minor)
    } else {
        // 上記以外の場合は、パッチ番号を比較
        Ok(new_patch > old_patch)
    }
}
