use crate::{
    duration::get_duration,
    types::{CountryCode, CountryDataset, DatasetEntry},
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::cell::OnceCell;
use walkdir::WalkDir;

pub struct US {
    cache: OnceCell<Vec<DatasetEntry>>,
}
impl US {
    pub fn new() -> Self {
        US {
            cache: OnceCell::new(),
        }
    }
}
impl CountryDataset for US {
    fn list_up(&self) -> Vec<DatasetEntry> {
        const DATASET_DIRECTORY: &str = "LibriSpeech/train-clean-100";

        if let Some(cache) = self.cache.get() {
            return cache.to_vec();
        }

        let result = WalkDir::new(DATASET_DIRECTORY)
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
                if entry.path().extension().unwrap().to_str().unwrap() != "txt" {
                    return None;
                }

                Some(
                    std::fs::read_to_string(entry.path())
                        .unwrap()
                        .lines()
                        .map(|line| {
                            let split_index = line.find(' ').unwrap();
                            let name = line[..split_index].to_string();
                            let transcription = line[split_index + 1..].to_string().to_lowercase();

                            let elements = name.split('-').collect::<Vec<_>>();
                            let path = format!(
                                "{}/{}/{}/{}.flac",
                                DATASET_DIRECTORY, elements[0], elements[1], name
                            );
                            let duration = get_duration(&path);

                            DatasetEntry::new(path, CountryCode::US, transcription, duration)
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .collect();
        self.cache.set(result).unwrap();
        return self.cache.get().unwrap().to_vec();
    }

    fn country_code(&self) -> CountryCode {
        CountryCode::US
    }
}
