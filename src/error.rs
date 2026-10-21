use std::{path, error, fmt};

pub type Result<T> = std::result::Result<T, SungaError>;

#[derive(Debug)]
pub enum SungaError {
    FileNotFound(String, path::PathBuf, String, u32),
    FileError(String, path::PathBuf, String, u32),
    ResizeError(String, path::PathBuf, String, u32),
    IndexError(usize, String, u32),
    InvalidVersion(String, u32),
    LockPoisoned(String, u32),
}

/// SungaError を表示
impl fmt::Display for SungaError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code();
        match self {
            SungaError::FileNotFound(e, p, _, _) => write!(fmt, "[{}] File does not exist: {}\n\n{}", code, e, p.display()),
            SungaError::FileError(e, p, _, _) => write!(fmt, "[{}] File error: {}\n\n{}", code, e, p.display()),
            SungaError::ResizeError(e, p, _, _) => write!(fmt, "[{}] Resize error: {}\n\n{}", code, e, p.display()),
            SungaError::IndexError(i, _, _) => write!(fmt, "[{}] Index error: {}", code, i),
            SungaError::InvalidVersion(_, _) => write!(fmt, "[{}] Invalid version", code),
            SungaError::LockPoisoned(_, _) => write!(fmt, "[{}] Lock poisoned", code),
        }
    }
}

/// SungaError を表示
impl error::Error for SungaError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SungaError::FileNotFound(_, _, _, _) => None,
            SungaError::FileError(_, _, _, _) => None,
            SungaError::ResizeError(_, _, _, _) => None,
            SungaError::IndexError(_, _, _) => None,
            SungaError::InvalidVersion(_, _) => None,
            SungaError::LockPoisoned(_, _) => None,
        }
    }
}

/// SungaError のメソッド
impl SungaError {
    /// 呼び出し元のファイル名と行番号
    /// * `return` - 呼び出し元のファイル名と行番号
    #[track_caller]
    fn caller() -> (String, u32) {
        let location = std::panic::Location::caller();
        (location.file().to_string(), location.line())
    }

    /// ファイルが見つからない
    /// * `message` - エラーメッセージ
    /// * `path` - ファイルパス
    /// * `return` - ファイルが見つからないエラー
    #[allow(dead_code)]
    #[track_caller]
    pub fn file_not_found(message: impl Into<String>, path: path::PathBuf) -> Self {
        let (file, line) = Self::caller();
        Self::FileNotFound(message.into(), path, file, line)
    }

    /// ファイルエラー
    /// * `message` - エラーメッセージ
    /// * `path` - ファイルパス
    /// * `return` - ファイルエラー
    #[track_caller]
    pub fn file_error(message: impl Into<String>, path: path::PathBuf) -> Self {
        let (file, line) = Self::caller();
        Self::FileError(message.into(), path, file, line)
    }
    /// リサイズエラー
    /// * `message` - エラーメッセージ
    /// * `path` - ファイルパス
    /// * `return` - リサイズエラー
    #[track_caller]
    pub fn resize_error(message: impl Into<String>, path: path::PathBuf) -> Self {
        let (file, line) = Self::caller();
        Self::ResizeError(message.into(), path, file, line)
    }

    /// インデックスエラー
    /// * `index` - インデックス
    /// * `return` - インデックスエラー
    #[allow(dead_code)]
    #[track_caller]
    pub fn index_error(index: usize) -> Self {
        let (file, line) = Self::caller();
        Self::IndexError(index, file, line)
    }

    /// バージョンが不正
    /// * `return` - バージョンが不正
    #[track_caller]
    pub fn invalid_version() -> Self {
        let (file, line) = Self::caller();
        Self::InvalidVersion(file, line)
    }

    /// ロックポイズンエラー
    /// * `return` - ロックポイズンエラー
    #[track_caller]
    pub fn lock_poisoned() -> Self {
        let (file, line) = Self::caller();
        Self::LockPoisoned(file, line)
    }

    /// 発生箇所のファイル名と行番号
    /// * `return` - ファイル名と行番号
    #[allow(dead_code)]
    pub fn location(&self) -> (&str, u32) {
        match self {
            Self::FileNotFound(_, _, file, line)
            | Self::FileError(_, _, file, line)
            | Self::ResizeError(_, _, file, line)
            | Self::IndexError(_, file, line)
            | Self::InvalidVersion(file, line)
            | Self::LockPoisoned(file, line)
            => (file, *line),
        }
    }

    /// エラーコード
    /// エラー項目・ファイル名・行番号を組み合わせて作る
    /// * `return` - エラーコード
    pub fn code(&self) -> String {
        match self {
            Self::FileNotFound(_, _, f, l) => format!("FNF_{}_{}", Self::file_to_code(f), l),
            Self::FileError(_, _, f, l) => format!("FE_{}_{}", Self::file_to_code(f), l),
            Self::ResizeError(_, _, f, l) => format!("RE_{}_{}", Self::file_to_code(f), l),
            Self::IndexError(_, f, l) => format!("IE_{}_{}", Self::file_to_code(f), l),
            Self::InvalidVersion(f, l) => format!("IVE_{}_{}", Self::file_to_code(f), l),
            Self::LockPoisoned(f, l) => format!("LP_{}_{}", Self::file_to_code(f), l),
        }
    }

    /// ファイルパスからコードを作る
    /// `src/` 以降を `/` と `_` で区切り、各語の頭文字をつなぐ
    /// 語が1つならその語をそのまま大文字にする。`mod` は除く
    /// * `file` - ファイルパス
    /// * `return` - コード
    fn file_to_code(file: &str) -> String {
        let normalized = file.replace('\\', "/");
        let relative = normalized
            .rsplit_once("src/")
            .map(|(_, rest)| rest)
            .unwrap_or(normalized.as_str());
        let words: Vec<&str> = relative
            .trim_end_matches(".rs")
            .split(['/', '_'])
            .filter(|word| !word.is_empty() && *word != "mod")
            .collect();

        match words.as_slice() {
            [] => "UNKNOWN".to_string(),
            [word] => word.to_uppercase(),
            words => words
                .iter()
                .filter_map(|word| word.chars().next())
                .flat_map(|ch| ch.to_uppercase())
                .collect(),
        }
    }
}

// tests ディレクトリ直下は別クレートになって、非公開関数が呼べないので、
// 子モジュールとして読むためのパスを指定する
#[cfg(test)]
#[path = "../tests/unit/error.rs"]
mod tests;
