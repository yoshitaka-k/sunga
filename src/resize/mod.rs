mod job;
pub(crate) use job::ResizeJob;

use std::path::Path;
use std::sync::Arc;
use std::sync:: Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashSet;
use crate::{error, file, resize};

/// リサイズトークン
#[derive(Clone)]
pub struct Token {
    pub id: u64,
    pub running: Arc<AtomicBool>,
    pub canceled: Arc<Mutex<HashSet<u64>>>,
}

impl Token {
    /// キャンセルされたかどうか
    /// * `return` - キャンセルされたかどうか
    pub fn is_canceled(&self) -> error::Result<bool> {
        Ok(!self.running.load(Ordering::Relaxed) || self.canceled.lock().map_err(|_| {
            error::ResizerError::lock_poisoned()
        })?.contains(&self.id))
    }
}

/// リサイズ
pub struct Resizer;

/// public methods
impl Resizer {
    /// リサイズ
    /// * `files` - リサイズするファイル
    /// * `return` - リサイズが成功したかどうか
    pub fn resize(path: &Path, token: &resize::Token) -> error::Result<file::Status> {
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        println!("TODO: resizing: {}", path.display());

        Ok(file::Status::Resized)
    }
}

/// private methods
impl Resizer {
}
