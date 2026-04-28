pub mod dispatch;
pub mod error;
pub mod event;
pub mod explore_bzl;
pub mod model;
pub mod task;
pub mod ui;

pub use error::Error;
pub type Result<T> = std::result::Result<T, error::Error>;
