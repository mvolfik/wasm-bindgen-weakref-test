You need to test with https://github.com/rustwasm/wasm-pack/pull/937

Build: `wasm-pack build --weak-refs --target web`

Serve current directory (e.g. with `python3 -m http.server . $PORT`)

Open localhost:$PORT/index.html in browser.

You can also enable wee_alloc by uncommenting it in `src/lib.rs`, but notice that it performs
poorly when it could do reallocations; most of the time, it just continues allocating on and on
