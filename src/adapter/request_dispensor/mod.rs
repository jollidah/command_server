#[allow(unused)]
pub mod vultr;

use reqwest::Client;
use std::sync::{Arc, OnceLock};
#[allow(unused)]
pub fn get_client() -> &'static Client {
    static CLIENT: OnceLock<Arc<Client>> = OnceLock::new();
    CLIENT.get_or_init(|| Arc::new(Client::new()))
}
