use crate::Config;

pub struct Answer {
    req: surf::Request<http_client::isahc::IsahcClient>,
    headers: Vec<(&'static str, String)>,
}
impl Answer {
    fn new(
        req: surf::Request<http_client::isahc::IsahcClient>,
        headers: Option<Vec<(&'static str, String)>>,
    ) -> Self {
        Self {
            req,
            headers: headers.unwrap_or_else(Vec::new),
        }
    }
    pub async fn json<T: serde::de::DeserializeOwned>(
        mut self,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync + 'static>> {
        for (key, val) in self.headers {
            self.req = self.req.set_header(key, val);
        }
        self.req.recv_json().await
    }
}

pub fn call<I: serde::Serialize>(conf: Config<I>) -> Answer {
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
            .body_string(val)
            .set_header("Content-Length", len.to_string())
            .set_header("Content-Type", "application/json")
    } else {
        client
    };

    Answer::new(v, conf.headers)
}
