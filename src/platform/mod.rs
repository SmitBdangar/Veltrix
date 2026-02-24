//! Platform abstractions: file system, system info, and clipboard.

pub mod clipboard;
pub mod filesystem;
pub mod system_info;

pub use filesystem::FileSystem;
pub use system_info::SystemInfo;
