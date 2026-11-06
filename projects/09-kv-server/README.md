# Project 9: Multi-threaded key-value web server

A tiny database you talk to over HTTP, like a shared notice board for
programs: anyone can pin a value under a name, read it, or take it down.
Built in Chapter 9 of *Rust in 199 Pages* to pull together threads,
`Arc<Mutex<..>>`, channels, TCP and JSON, using only the standard library
plus `serde_json`.

```bash
cargo run -p kv-server -- --port 7878 --threads 4
# in another terminal
curl -X PUT -d Pune localhost:7878/keys/city     # {"created":true,"key":"city"}
curl localhost:7878/keys/city                    # {"key":"city","value":"Pune"}
curl localhost:7878/keys                         # {"keys":["city"]}
curl -X DELETE localhost:7878/keys/city          # {"deleted":"Pune","key":"city"}
curl localhost:7878/stats                        # {"keys":0,"requests":5}
cargo test -p kv-server
```

## API

| Method | Path          | Answer                                  |
|--------|---------------|-----------------------------------------|
| GET    | /health       | 200 `{"status":"ok"}`                   |
| GET    | /keys         | 200 sorted list of keys                 |
| GET    | /keys/{key}   | 200 value, or 404                       |
| PUT    | /keys/{key}   | body is the value; 201 new, 200 replaced|
| DELETE | /keys/{key}   | 200 old value, or 404                   |
| GET    | /stats        | 200 key count and request count         |

Keys use letters, digits, `-` and `_`, up to 64 characters (else 400).
Bodies over 64 KB get 413. Other methods on a key get 405.

## Layout

```text
09-kv-server/
├── src/
│   ├── main.rs     options, the accept loop, one job per connection
│   ├── lib.rs      lists the modules so tests can use them
│   ├── http.rs     reads a Request, writes a Response; unit tests
│   ├── store.rs    Store: Arc<Mutex<HashMap>> + an atomic request counter
│   ├── pool.rs     ThreadPool: N workers sharing one channel receiver
│   └── router.rs   (method, path) -> action; unit tests
└── tests/
    └── server.rs   starts the real binary and sends real TCP requests,
                    including 20 clients at once
```
