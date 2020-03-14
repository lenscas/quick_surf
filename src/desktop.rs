use crate::Config;

pub struct Answer {
    req: surf::Request<http_client::isahc::IsahcClient>,
}
impl Answer {
    fn new(req: surf::Request<http_client::isahc::IsahcClient>) -> Self {
        Self { req }
    }
    pub async fn json<T: serde::de::DeserializeOwned>(
        self,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.req.recv_json().await
    }
}

pub async fn call<I: serde::Serialize>(conf: Config<I>) -> Answer {
    let client = surf::Client::new();
    let client = {
        use crate::Method::*;
        match conf.method {
            Post => client.post(conf.url),
            Put => client.put(conf.url),
            Get => client.get(conf.url),
            Delete => client.delete(conf.url),
        }
    };
    let v = if let Some(body) = conf.body {
        let val = serde_json::to_string(&body).unwrap();

        let len = val.len();
        client
            .set_header("Content-Length", len.to_string())
            .set_header("Content-Type", "application/json")
            .body_string(val)
    } else {
        client
    };

    Answer::new(v)
}
