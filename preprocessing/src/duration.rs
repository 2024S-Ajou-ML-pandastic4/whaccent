use std::process::Command;

pub fn get_duration(path: &str) -> f64 {
    let mut command = Command::new("ffprobe");
    command.args([
        "-i",
        path,
        "-show_entries",
        "format=duration",
        "-v",
        "quiet",
        "-of",
        "csv=p=0",
    ]);
    let output = String::from_utf8(command.output().unwrap().stdout).unwrap();
    let duration = output.trim().parse::<f64>().unwrap();
    duration
}
