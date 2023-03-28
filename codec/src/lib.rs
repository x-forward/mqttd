pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub mod frame;
pub mod parse;
