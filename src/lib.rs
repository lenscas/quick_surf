#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::{call, Config, Method};
        let _t = call::<()>(Config {
            url: "127.0.0.1".into(),
            method: Method::Get,
            body: None,
            headers: None,
        });
    }
}
pub enum Method {
    Post,
    Put,
    Get,
    Delete,
}
pub struct Config<I: serde::Serialize> {
    pub url: String,
    pub method: Method,
    pub body: Option<I>,
    pub headers: Option<Vec<(&'static str, String)>>,
}

#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(not(target_arch = "wasm32"))]
pub use desktop::{call, Answer};

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
extern crate stdweb;
#[cfg(target_arch = "wasm32")]
pub use web::{call, Answer};
