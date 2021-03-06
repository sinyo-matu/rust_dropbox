pub mod client;
mod test;

use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct DbxRequestLimitsErrorSummary {
    error_summary: String,
    error: DbxRequestErrorReason,
}
#[derive(Debug, Deserialize)]
struct DbxRequestErrorSummary {
    error_summary: String,
    error: DbxRequestErrorTag,
}

#[derive(Debug, Deserialize)]
struct DbxRequestErrorReason {
    reason: DbxRequestErrorTag,
    retry_after: u32,
}
#[derive(Debug, Deserialize)]
struct DbxRequestErrorTag {
    #[serde(alias = ".tag")]
    tag: String,
}
#[derive(Debug, Deserialize)]
struct UserCheckResult {
    result: String,
}

pub type DropboxResult<T> = std::result::Result<T, DropboxError>;
#[derive(Debug)]
pub enum DropboxError {
    #[cfg(feature = "non-blocking")]
    NonBlockingRequestError(reqwest::Error),
    #[cfg(feature = "blocking")]
    BlockingRequestError(ureq::Error),
    DbxUserCheckError(String),
    DbxPathError(String),
    DbxExistedError(String),
    DbxInvalidTokenError(String),
    DbxFromLookUpError(String),
    DbxRequestLimitsError(String),
    DbxAccessError(String),
    DbxConflictError(String),
    DbxServerError(String),
    OtherError(String),
    #[cfg(feature = "blocking")]
    BodyParseError(std::io::Error),
}

#[cfg(feature = "blocking")]
impl From<std::io::Error> for DropboxError {
    fn from(e: std::io::Error) -> Self {
        Self::BodyParseError(e)
    }
}

#[cfg(feature = "non-blocking")]
impl From<reqwest::Error> for DropboxError {
    fn from(e: reqwest::Error) -> Self {
        Self::NonBlockingRequestError(e)
    }
}
#[cfg(feature = "blocking")]
impl From<ureq::Error> for DropboxError {
    fn from(e: ureq::Error) -> Self {
        Self::BlockingRequestError(e)
    }
}
pub struct MoveOption {
    allow_shared_folder: bool,
    auto_rename: bool,
    allow_ownership_transfer: bool,
}

impl MoveOption {
    pub fn new() -> Self {
        Self {
            allow_shared_folder: false,
            auto_rename: false,
            allow_ownership_transfer: false,
        }
    }

    pub fn allow_shared_folder(mut self) -> Self {
        self.allow_shared_folder = true;
        self
    }

    pub fn allow_auto_rename(mut self) -> Self {
        self.auto_rename = true;
        self
    }

    pub fn allow_ownership_transfer(mut self) -> Self {
        self.allow_ownership_transfer = true;
        self
    }
}
pub enum UploadMode {
    Add,
    Overwrite,
}
