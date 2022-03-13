pub mod api;
pub use api::*;

/// Returns the api endpoint
pub fn api_endpoint() -> String {
    "https://api.migadu.com/v1/domains".to_string()
}
