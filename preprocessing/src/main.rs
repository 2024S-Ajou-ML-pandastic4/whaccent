mod country;
mod duration;
mod encode;
mod export_manifest;
mod types;

use crate::types::CountryDataset;
use country::{GB, KR, US};
use encode::encode_dataset;
use export_manifest::export_manifest;
use rand::{seq::SliceRandom, thread_rng};

const SKIP_EXPORT_MANIFEST: bool = true;
const SKIP_ENCODE: bool = true;
const SAMPLE_PER_COUNTRY: usize = 11352;
const TEST_RATIO: f32 = 0.2;

fn main() {
    let countries: Vec<Box<dyn CountryDataset>> = vec![
        Box::new(KR::new()),
        Box::new(US::new()),
        Box::new(GB::new()),
    ];

    if !SKIP_EXPORT_MANIFEST {
        for country in countries.iter() {
            let mut dataset = country.list_up();
            dataset.sort_by(|a, b| a.duration.partial_cmp(&b.duration).unwrap());
            dataset.truncate(SAMPLE_PER_COUNTRY);
            dataset.shuffle(&mut thread_rng());

            let test_size = (dataset.len() as f32 * TEST_RATIO) as usize;
            let (test_dataset, train_dataset) = dataset.split_at(test_size);

            let train_output_path =
                format!("output/{}_train.csv", country.country_code().to_string());
            let test_output_path =
                format!("output/{}_test.csv", country.country_code().to_string());

            export_manifest(&train_output_path, &train_dataset);
            export_manifest(&test_output_path, &test_dataset);
        }
    }

    if !SKIP_ENCODE {
        for country in countries.iter() {
            let dataset = country.list_up();
            encode_dataset(&dataset);
        }
    }
}
