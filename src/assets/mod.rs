//! Asset server: typed handles, cache, and hot-reloading.

pub mod cache;
pub mod handle;
pub mod loader;
pub mod server;

pub use handle::Handle;
pub use server::AssetServer;
