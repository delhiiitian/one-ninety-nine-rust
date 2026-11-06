//! Maps (method, path) to an action on the store.
//!
//!   GET    /health        -> {"status":"ok"}
//!   GET    /keys          -> {"keys":[...]}
//!   GET    /keys/{key}    -> {"key":..,"value":..}   or 404
//!   PUT    /keys/{key}    -> body is the value; 201 new, 200 replaced
//!   DELETE /keys/{key}    -> 200 or 404
//!   GET    /stats         -> {"keys":n,"requests":n}

use crate::http::{Request, Response};
use crate::store::{SetResult, Store};
use serde_json::json;

pub fn route(req: &Request, store: &Store) -> Response {
    store.count_request();
    let segments: Vec<&str> = req.path.trim_matches('/').split('/').collect();

    match (req.method.as_str(), segments.as_slice()) {
        ("GET", ["health"]) => Response::json(200, json!({"status": "ok"})),
        ("GET", ["stats"]) => Response::json(200, json!({"keys": store.len(), "requests": store.requests()})),
        ("GET", ["keys"]) => Response::json(200, json!({"keys": store.keys()})),

        (method, ["keys", key]) if valid_key(key) => match method {
            "GET" => match store.get(key) {
                Some(value) => Response::json(200, json!({"key": key, "value": value})),
                None => not_found(key),
            },
            "PUT" => match store.set(key, &req.body) {
                SetResult::Created => Response::json(201, json!({"key": key, "created": true})),
                SetResult::Updated(old) => Response::json(200, json!({"key": key, "created": false, "old": old})),
            },
            "DELETE" => match store.delete(key) {
                Some(old) => Response::json(200, json!({"key": key, "deleted": old})),
                None => not_found(key),
            },
            _ => Response::json(405, json!({"error": format!("{method} is not allowed here")})),
        },
        (_, ["keys", _]) => Response::json(400, json!({"error": "keys use letters, digits, - and _ (max 64)"})),

        _ => Response::json(404, json!({"error": format!("no route for {} {}", req.method, req.path)})),
    }
}

fn valid_key(key: &str) -> bool {
    !key.is_empty() && key.len() <= 64 && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn not_found(key: &str) -> Response {
    Response::json(404, json!({"error": format!("no key named {key}")}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn req(method: &str, path: &str, body: &str) -> Request {
        Request { method: method.into(), path: path.into(), headers: HashMap::new(), body: body.into() }
    }

    #[test]
    fn full_life_of_a_key() {
        let s = Store::new();
        assert_eq!(route(&req("PUT", "/keys/city", "Pune"), &s).status, 201);
        assert_eq!(route(&req("PUT", "/keys/city", "Delhi"), &s).body, r#"{"created":false,"key":"city","old":"Pune"}"#);
        assert_eq!(route(&req("GET", "/keys/city", ""), &s).body, r#"{"key":"city","value":"Delhi"}"#);
        assert_eq!(route(&req("GET", "/keys", ""), &s).body, r#"{"keys":["city"]}"#);
        assert_eq!(route(&req("DELETE", "/keys/city", ""), &s).status, 200);
        assert_eq!(route(&req("GET", "/keys/city", ""), &s).status, 404);
        assert_eq!(route(&req("GET", "/stats", ""), &s).body, r#"{"keys":0,"requests":7}"#);
    }

    #[test]
    fn bad_requests() {
        let s = Store::new();
        assert_eq!(route(&req("POST", "/keys/a", ""), &s).status, 405);
        assert_eq!(route(&req("GET", "/keys/no%20spaces", ""), &s).status, 400);
        assert_eq!(route(&req("GET", "/nothing", ""), &s).status, 404);
    }
}
