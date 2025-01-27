use super::azik_config::AzikConfig;
use clap::Parser;
use std::{fs::File, io::BufReader};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

pub fn azik_deserializer() -> AzikConfig {
    let path = Cli::parse().path;
    let file = File::open(path).expect("Can't read file");
    let reader: BufReader<File> = BufReader::new(file);
    let config: Result<AzikConfig, serde_json::Error> = serde_json::from_reader(reader);

    config.expect("Not support this format")
}
