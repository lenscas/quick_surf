#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::{call, Config, Method};
        let _t = call::<()>(Config {
            url: "http://httpbin.org/get".into(),
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

#[derive(Debug)]
pub enum Error {
    BadHeader(&'static str),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadHeader(x) => x.fmt(f),
        }
    }
}
impl std::error::Error for Error {}

pub struct Config<I: serde::Serialize> {
    pub url: String,
    pub method: Method,
    pub body: Option<I>,
    pub headers: Option<Vec<(&'static str, String)>>,
}

#[cfg(all(not(target_arch = "wasm32"), feature = "std-web"))]
mod desktop;
#[cfg(all(not(target_arch = "wasm32"), feature = "std-web"))]
pub use desktop::{call, Answer};

#[cfg(all(target_arch = "wasm32", feature = "std-web"))]
mod web;
#[cfg(all(target_arch = "wasm32", feature = "std-web"))]
extern crate stdweb;
#[cfg(all(target_arch = "wasm32", feature = "std-web"))]
pub use web::{call, Answer};
