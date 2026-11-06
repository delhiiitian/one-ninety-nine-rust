//! A small multi-threaded key-value web server.
//!
//! - `http`   reads requests and writes responses (HTTP/1.1, by hand)
//! - `store`  the shared data: `Arc<Mutex<HashMap>>` behind a clean API
//! - `pool`   a fixed set of worker threads fed by a channel
//! - `router` turns a request into a response

pub mod http;
pub mod pool;
pub mod router;
pub mod store;
