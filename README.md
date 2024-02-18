## Base Rust Web API

## Run

The quick_dev.rs file execute the HTTP request and output the result of the request.

Run quick_dev.rs

```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

Open an run in a second terminal panel to simulate hot reloading

```sh
cargo watch -q -c -w src/ -x run
```
