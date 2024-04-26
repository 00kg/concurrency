// metrics data structure
// 基本功能： inc/dec/snapshot

use anyhow::Result;
use core::fmt;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // let mut data = self
        //     .data
        //     .write()
        //     .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        // let mut data = self
        //     .data
        //     .write()
        //     .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let data = self.data.read().map_err(|_| fmt::Error {})?;
        for entry in self.data.iter() {
            writeln!(f, "{}, {}", entry.key(), entry.value())?;
        }
        Ok::<_, fmt::Error>(())
    }
}
