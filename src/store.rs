use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct KeyValueStore {
    pub map: Mutex<HashMap<String, String>>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let map = self.map.lock().await;
        map.get(key).cloned()
    }

    pub async fn set(&self, key: String, value: String) -> anyhow::Result<()> {
        {
            let mut map = self.map.lock().await;
            map.insert(key, value);
        }
        self.save("data.json").await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> anyhow::Result<bool> {
        let removed = {
            let mut map = self.map.lock().await;
            map.remove(key).is_some()
        }; // Lock is released here

        if removed {
            self.save("data.json").await?;
        }
        Ok(removed)
    } 
}