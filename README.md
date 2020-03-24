# quick_surf
Made for use in [card_game](https://github.com/lenscas/card_game_client).

It is simply a wrapper arround surf for native builds and a wrapper arround the fetch api when building to wasm.
The reason is simply because although surf supports wasm builds, it uses wasm-bindgen while card_game uses stdweb.

It also replaces surf's request factory pattern with a simple config struct, which I personally like more.
