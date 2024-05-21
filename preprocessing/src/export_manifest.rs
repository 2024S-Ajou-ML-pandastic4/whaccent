use crate::types::{DatasetEntry, ManifestRecord};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn export_manifest(output_path: &str, dataset: &[DatasetEntry]) {
    let mut writer = csv::Writer::from_path(output_path).unwrap();
    let records = dataset
        .par_iter()
        .map(|data| ManifestRecord::from(data))
        .collect::<Vec<_>>();
    for record in records {
        writer.serialize(&record).unwrap();
    }
    writer.flush().unwrap();
}
