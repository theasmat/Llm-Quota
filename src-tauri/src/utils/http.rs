use once_cell::sync::Lazy;
use reqwest::Client;

/// Global shared HTTP client (15s timeout)
pub static SHARED_CLIENT: Lazy<Client> = Lazy::new(|| create_base_client(15));

/// Global shared HTTP client (Long timeout: 60s)
pub static SHARED_CLIENT_LONG: Lazy<Client> = Lazy::new(|| create_base_client(60));

/// Global shared standard HTTP client (15s timeout)
pub static SHARED_STANDARD_CLIENT: Lazy<Client> = Lazy::new(|| create_base_client(15));

/// Global shared standard HTTP client (Long timeout: 60s)
pub static SHARED_STANDARD_CLIENT_LONG: Lazy<Client> = Lazy::new(|| create_base_client(60));

/// Base client creation logic
fn create_base_client(timeout_secs: u64) -> Client {
    let builder = Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs));

    builder.build().unwrap_or_else(|_| Client::new())
}

/// Get uniformly configured HTTP client (15s timeout)
pub fn get_client() -> Client {
    SHARED_CLIENT.clone()
}

/// Get long timeout HTTP client (60s timeout)
pub fn get_long_client() -> Client {
    SHARED_CLIENT_LONG.clone()
}

/// Get standard HTTP client (15s timeout)
pub fn get_standard_client() -> Client {
    SHARED_STANDARD_CLIENT.clone()
}

/// Get long timeout standard HTTP client (60s timeout)
pub fn get_long_standard_client() -> Client {
    SHARED_STANDARD_CLIENT_LONG.clone()
}
