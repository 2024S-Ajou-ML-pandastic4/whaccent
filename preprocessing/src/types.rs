use serde::{Deserialize, Serialize};
use sha256::digest;
use std::path::Path;

pub trait CountryDataset {
    fn list_up(&self) -> Vec<DatasetEntry>;
    fn country_code(&self) -> CountryCode;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatasetEntry {
    pub id: String,
    pub path: String,
    pub country: CountryCode,
    pub transcription: String,
    pub duration: f64,
}
impl DatasetEntry {
    pub fn new(path: String, country: CountryCode, transcription: String, duration: f64) -> Self {
        let id = digest(Path::new(&path).to_str().unwrap().to_string());
        DatasetEntry {
            id,
            path,
            country,
            transcription,
            duration,
        }
    }
    pub fn encoded_path(&self) -> String {
        format!("output/audio/{}/{}.flac", &self.id[..2], &self.id[2..])
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CountryCode {
    GB,
    KR,
    US,
}
impl ToString for CountryCode {
    fn to_string(&self) -> String {
        match self {
            CountryCode::GB => "GB".to_string(),
            CountryCode::KR => "KR".to_string(),
            CountryCode::US => "US".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestRecord {
    pub id: String,
    pub audio_path: String,
    pub country: CountryCode,
    pub transcription: String,
    pub duration: f64,
}
impl From<&DatasetEntry> for ManifestRecord {
    fn from(value: &DatasetEntry) -> Self {
        let audio_path = value.encoded_path();
        Self {
            id: value.id.clone(),
            audio_path,
            country: value.country,
            transcription: value.transcription.clone(),
            duration: value.duration,
        }
    }
}
