use crate::types::DatasetEntry;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

pub fn encode_dataset(dataset: &[DatasetEntry]) {
    dataset.par_iter().for_each(|entry| {
        fs::create_dir_all(Path::new(&entry.encoded_path()).parent().unwrap()).unwrap();
        let gain = -get_max_volume(&entry.path);
        let mut command = Command::new("ffmpeg");
        command.stderr(Stdio::null());
        command.stdout(Stdio::null());
        command.args([
            "-y",
            "-i",
            &entry.path,
            "-af",
            &format!("volume={gain}"),
            "-c:a",
            "flac",
            "-sample_fmt",
            "s16",
            "-r",
            "16000",
            &entry.encoded_path(),
        ]);
        let status = command.spawn().unwrap().wait().unwrap();
        assert!(status.success());
    });
}

pub fn get_max_volume(input: &str) -> f64 {
    let mut command = Command::new("ffmpeg");
    command.args([
        "-i",
        input,
        "-af",
        "volumedetect",
        "-vn",
        "-sn",
        "-dn",
        "-f",
        "null",
        "/dev/null",
    ]);
    let output = String::from_utf8(command.output().unwrap().stderr).unwrap();
    let max_volume = output
        .lines()
        .find(|line| line.contains("max_volume"))
        .unwrap()
        .split(' ')
        .rev()
        .skip(1)
        .next()
        .unwrap()
        .parse::<f64>()
        .unwrap();
    max_volume
}
