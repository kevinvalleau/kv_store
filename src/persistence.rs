use crate::store::KeyValueStore;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

#[derive(Serialize, Deserialize)]
struct PersistantData {
    data: Vec<(String, String)>,
}

impl KeyValueStore {
    pub async fn load(&self, path: &str) -> anyhow::Result<()> {
        if Path::new(path).exists() {
            let content = tokio::fs::read_to_string(path).await?;
            let persistent_data: PersistantData = serde_json::from_str(&content)?;
            let mut map = self.map.lock().await;
            map.extend(persistent_data.data);
        }
        Ok(())
    }

    pub async fn save(&self, path: &str) -> anyhow::Result<()> {
        let data = {
            let map = self.map.lock().await;
            PersistantData {
                data: map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            }
        }; // Lock is released here
        let content = serde_json::to_string(&data)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
}