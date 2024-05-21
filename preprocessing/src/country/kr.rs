use crate::{
    duration::get_duration,
    types::{CountryCode, CountryDataset, DatasetEntry},
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::Deserialize;
use std::cell::OnceCell;
use walkdir::WalkDir;

pub struct KR {
    cache: OnceCell<Vec<DatasetEntry>>,
}
impl KR {
    pub fn new() -> Self {
        KR {
            cache: OnceCell::new(),
        }
    }
}
impl CountryDataset for KR {
    fn list_up(&self) -> Vec<DatasetEntry> {
        const AUDIO_DIRECTORY: &str = "Validation/01.원천데이터";
        const MANIFEST_DIRECTORY: &str = "Validation/02.라벨링데이터";

        if let Some(cache) = self.cache.get() {
            return cache.to_vec();
        }

        let dataset: Vec<DatasetEntry> = WalkDir::new(MANIFEST_DIRECTORY)
            .into_iter()
            .par_bridge()
            .filter_map(|entry| {
                let Ok(entry) = entry else {
                    print!("Error: {:?}", entry);
                    return None;
                };
                if !entry.file_type().is_file() {
                    return None;
                }
                if entry.path().extension().unwrap().to_str().unwrap() != "json" {
                    return None;
                }

                let manifest_string = std::fs::read_to_string(entry.path()).unwrap();
                let manifest: ManifestEntry =
                    serde_json::from_str(manifest_string.trim_start_matches("\u{feff}")).unwrap();
                let path = format!("{}/{}.wav", AUDIO_DIRECTORY, manifest.file_nm);
                let duration = get_duration(&path);
                Some(DatasetEntry::new(
                    path,
                    CountryCode::KR,
                    manifest.item_script.contents.into_iter().fold(
                        String::new(),
                        |mut acc, content| {
                            acc.push(' ');
                            acc.push_str(&content.sentence);
                            acc
                        },
                    ),
                    duration,
                ))
            })
            .collect();
        self.cache.set(dataset).unwrap();
        return self.cache.get().unwrap().to_vec();
    }
    fn country_code(&self) -> CountryCode {
        CountryCode::KR
    }
}

#[derive(Debug, Deserialize)]
struct ManifestEntry {
    #[serde(rename = "fileNm")]
    file_nm: String,
    #[serde(rename = "itemScript")]
    item_script: ItemScript,
}

#[derive(Debug, Deserialize)]
struct ItemScript {
    contents: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    sentence: String,
}
