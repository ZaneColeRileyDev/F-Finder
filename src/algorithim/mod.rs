mod search;
mod models;

pub use models::{OutputPath, OutputJson};
#[cfg(target_os = "windows")]
pub use search::{search_file_by_name, search_file_by_ext, search_file_by_both, get_roots};
#[cfg(target_os = "linux")]
pub use search::{search_file_by_name, search_file_by_ext, search_file_by_both};
