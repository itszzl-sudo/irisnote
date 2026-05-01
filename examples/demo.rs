use std::collections::HashMap;

struct Cache {
    data: HashMap<String, String>,
    capacity: usize,
}

impl Cache {
    fn new(capacity: usize) -> Self {
        Cache {
            data: HashMap::with_capacity(capacity),
            capacity,
        }
    }
    
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    fn insert(&mut self, key: String, value: String) {
        if self.data.len() >= self.capacity {
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.remove(&first_key);
            }
        }
        self.data.insert(key, value);
    }
}

fn main() {
    let mut cache = Cache::new(3);
    
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    cache.insert("key3".to_string(), "value3".to_string());
    
    if let Some(value) = cache.get("key1") {
        println!("Found: {}", value);
    }
    
    println!("Cache size: {}", cache.data.len());
}
