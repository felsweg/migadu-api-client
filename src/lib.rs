pub mod api;
pub use api::*;

// pub const ENV_MIGADU_APIKEY: &str = "MIGADU_APIKEY";
// pub const ENV_MIGADU_USER: &str = "MIGADU_USER";

pub fn api_endpoint() -> String {
    "https://api.migadu.com/v1/domains".to_string()
}
