use std::io::{Error, Write};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OutputPath {
    path: String,
    id: i32,
    #[serde(with = "humantime_serde")]
    timestamp: SystemTime,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OutputJson {
    paths: Vec<OutputPath>
}


impl OutputPath {

    pub fn new(path: String, id: i32) -> OutputPath {
        OutputPath {
            path,
            id,
            timestamp: SystemTime::now(),
        }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }
}

impl OutputJson {

    pub fn new(paths: Vec<OutputPath>) -> OutputJson {
        OutputJson {
            paths
        }
    }
    pub fn get_output_path_by_id(&self, id: i32) -> Option<OutputPath> {
        self.paths.iter().find(|p| p.id() == id).map(|p| p.clone())
    }

    pub fn output_paths(&self) -> Vec<OutputPath> {
        self.paths.clone()
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(&self.paths).unwrap_or_else(|_| serde_json::Value::Null)
    }

    pub fn output_json_file(&self, path: String) -> Result<(), Error> {
        if let Some(parent) = std::path::Path::new(&path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::File::create(&path)?;
        let content = serde_json::to_string_pretty(&self.paths)?;
        file.write_all(content.as_bytes())
    }
}