use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
struct Peer {
    id: u32,
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl Peer {
    fn new(id: u32) -> Self {
        Peer {
            id,
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Insert a key-value pair into this peer's store
    fn insert(&self, key: String, value: String) {
        let mut db = self.store.lock().unwrap();
        db.insert(key.clone(), value.clone());
        println!("Peer {} stored: {} -> {}", self.id, key, value);
    }

    // Lookup key locally
    fn lookup(&self, key: &String) -> Option<String> {
        let db = self.store.lock().unwrap();
        db.get(key).cloned()
    }
}

#[tokio::main]
async fn main() {
    // Create a small network of peers
    let mut peers: Vec<Peer> = (1..=5).map(Peer::new).collect();

    // Randomly assign some key-value pairs to peers
    let sample_data = vec![
        ("alice".to_string(), "100".to_string()),
        ("bob".to_string(), "250".to_string()),
        ("carol".to_string(), "300".to_string()),
    ];

    for (key, value) in sample_data {
        let idx = rand::thread_rng().gen_range(0..peers.len());
        peers[idx].insert(key, value);
    }

    // Simulate lookups from random peers
    let queries = vec!["alice".to_string(), "bob".to_string(), "dave".to_string()];

    for query in queries {
        let idx = rand::thread_rng().gen_range(0..peers.len());
        println!("\nPeer {} querying for key: {}", peers[idx].id, query);

        // Brute-force: query all peers until found
        let mut found = None;
        for peer in &peers {
            if let Some(val) = peer.lookup(&query) {
                found = Some(val);
                break;
            }
        }

        match found {
            Some(v) => println!("✅ Key '{}' found with value: {}", query, v),
            None => println!("❌ Key '{}' not found in the network", query),
        }

        sleep(Duration::from_millis(500)).await;
    }
}
