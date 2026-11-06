//! The shared key-value data. Every worker thread holds a clone of `Store`;
//! all clones point at the same map, guarded by one Mutex.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct Store {
    data: Arc<Mutex<HashMap<String, String>>>,
    requests: Arc<AtomicU64>,
}

/// What `set` did, so the router can answer 201 or 200.
#[derive(Debug, PartialEq)]
pub enum SetResult {
    Created,
    Updated(String), // carries the old value
}

impl Store {
    pub fn new() -> Store {
        Store::default()
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().get(key).cloned()
    }

    pub fn set(&self, key: &str, value: &str) -> SetResult {
        match self.data.lock().unwrap().insert(key.to_string(), value.to_string()) {
            None => SetResult::Created,
            Some(old) => SetResult::Updated(old),
        }
    }

    pub fn delete(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().remove(key)
    }

    /// All keys, sorted so answers are stable.
    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.data.lock().unwrap().keys().cloned().collect();
        keys.sort();
        keys
    }

    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Counts a request. An atomic number needs no lock at all.
    pub fn count_request(&self) -> u64 {
        self.requests.fetch_add(1, Ordering::Relaxed) + 1
    }

    pub fn requests(&self) -> u64 {
        self.requests.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn set_get_delete() {
        let s = Store::new();
        assert_eq!(s.set("tea", "20"), SetResult::Created);
        assert_eq!(s.set("tea", "25"), SetResult::Updated("20".into()));
        assert_eq!(s.get("tea").as_deref(), Some("25"));
        assert_eq!(s.delete("tea").as_deref(), Some("25"));
        assert!(s.is_empty());
    }

    #[test]
    fn many_threads_share_one_map() {
        let s = Store::new();
        let handles: Vec<_> = (0..8)
            .map(|t| {
                let s = s.clone();
                thread::spawn(move || {
                    for i in 0..100 {
                        s.set(&format!("t{t}-{i}"), "x");
                        s.count_request();
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(s.len(), 800);
        assert_eq!(s.requests(), 800);
    }
}
