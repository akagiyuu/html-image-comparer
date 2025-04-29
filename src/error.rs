pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Browser error: {0}")]
    Browser(#[from] chromiumoxide::error::CdpError),

    #[error("{0}")]
    Image(&'static str),
}
