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
        let v = self.req.recv_string().await?;
        let v = serde_json::from_str(&v)?;
        Ok(v)
    }
    pub async fn bytes(
        self,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let v = self.req.recv_bytes().await?;
        Ok(v)
    }
}

pub fn call<I: serde::Serialize>(conf: Config<I>) -> Result<Answer, crate::Error> {
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
    let mut client = if let Some(body) = conf.body {
        let val = serde_json::to_string(&body).unwrap();

        let len = val.len();
        let client = client
            .body_string(val)
            .set_header("Content-Length".parse().unwrap(), len.to_string())
            .set_header("Content-Type".parse().unwrap(), "application/json");
        client
    } else {
        client
    };
    if let Some(headers) = conf.headers {
        for (key, value) in headers {
            let key = dbg!(key);
            let value = dbg!(value);
            let checked_key = key.parse();

            client = match checked_key {
                Ok(key) => client.set_header(key, value),
                Err(_) => return Err(crate::Error::BadHeader(key)),
            }
        }
    }
    Ok(Answer::new(client))
}
