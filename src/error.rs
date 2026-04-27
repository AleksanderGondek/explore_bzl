#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Imaginary")]
  Imaginary,
  #[error("IO Error.\nCause:\n  {0}")]
  IoError(#[from] std::io::Error),
}
