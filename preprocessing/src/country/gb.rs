use crate::{
    duration::get_duration,
    types::{CountryCode, CountryDataset, DatasetEntry},
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::Deserialize;
use std::cell::OnceCell;

pub struct GB {
    cache: OnceCell<Vec<DatasetEntry>>,
}
impl GB {
    pub fn new() -> Self {
        GB {
            cache: OnceCell::new(),
        }
    }
}
impl CountryDataset for GB {
    fn list_up(&self) -> Vec<DatasetEntry> {
        const AUDIO_DIRECTORY: &str = "clarity_utterances/audio";
        const MANIFEST_PATH: &str = "clarity_utterances/clarity_master.json";

        if let Some(cache) = self.cache.get() {
            return cache.to_vec();
        }

        let manifest: Vec<ManifestEntry> =
            serde_json::from_str(std::fs::read_to_string(MANIFEST_PATH).unwrap().as_str()).unwrap();
        let result = manifest
            .into_iter()
            .par_bridge()
            .map(|entry| {
                let path = format!("{}/{}.wav", AUDIO_DIRECTORY, entry.wavfile);
                let duration = get_duration(&path);
                DatasetEntry::new(path, CountryCode::GB, entry.prompt, duration)
            })
            .collect();
        self.cache.set(result).unwrap();
        return self.cache.get().unwrap().to_vec();
    }
    fn country_code(&self) -> CountryCode {
        CountryCode::GB
    }
}

#[derive(Debug, Deserialize)]
struct ManifestEntry {
    prompt: String,
    wavfile: String,
}
