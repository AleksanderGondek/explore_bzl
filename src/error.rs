#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Imaginary")]
  Imaginary,
  #[error("IO Error.\nCause:\n  {0}")]
  IoError(#[from] std::io::Error),
  #[error("Proto decode error.\n Cause: {0}")]
  ProtoDecodeError(#[from] prost::DecodeError),
}
