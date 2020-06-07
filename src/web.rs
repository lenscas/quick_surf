use crate::Config;
use stdweb::{PromiseFuture,js,unstable::TryInto,web::ArrayBuffer};

pub struct Answer {
    value : PromiseFuture<ArrayBuffer>
}
impl Answer {
    fn new(value : PromiseFuture<ArrayBuffer>) -> Self {
        Self {value}
    }
    pub async fn json<T: serde::de::DeserializeOwned>(
        self,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync + 'static>> {
        //self.req.recv_json().await
        Ok(serde_json::from_slice(&Vec::<_>::from(self.value.await?))?)
    }
    pub async fn bytes(self) -> Result<Vec<u8>,Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(self.value.await?.into())
    }
}
pub fn call<T : serde::Serialize>(conf : Config<T>) -> Result<Answer,crate::Error> {
    let method : String = {
        use crate::Method::*;
        match conf.method {
            Get => "GET".into(),
            Post => "POST".into(),
            Put => "PUT".into(),
            Delete => "DELETE".into()
        }
    };
    let v = if let Some(body) = conf.body {
        serde_json::to_string(&body).unwrap()
    } else {
        "".into()
    };
    let has_body = &v != "";

    let headers : stdweb::Array = conf.headers
        .unwrap_or_else(Vec::new)
        .into_iter()
        .map(|v|vec![v.0.to_owned(),v.1])
        .collect::<Vec<_>>().into();

    let res : PromiseFuture<ArrayBuffer> = js! {
        const config = {
            method : @{method},
            headers: {
                "Content-Type": "application/json"
                // 'Content-Type': 'application/x-www-form-urlencoded',
            }
        };
        @{headers}.forEach(v=>config.headers[v[0]] = v[1]);
        console.log(@{has_body});
        if(@{has_body}) {
            config.body = @{v}
        }
        console.log(config);
        return fetch(@{conf.url},config).then(v=>v.arrayBuffer())
    }.try_into().unwrap();
    Ok(Answer::new(res))
}