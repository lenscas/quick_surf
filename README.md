# Silver_surf
Made for use in [card_game](https://github.com/lenscas/card_game_client).

Silver surf is a wrapper around surf to get rid of the builder pattern that surf uses and replaces it with a config struct.
Optionally, you can enable the stdweb wrapper if you want to build for the web but don't want to use wasm-bindgen.

```rust
let res = call::<()>(Config {
    url: "your_url".into(),
    method: Method::Get,
    body: None,
    headers: None,
}).json::<YourStruct>().await;
```
