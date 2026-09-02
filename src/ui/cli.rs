use clap::Parser;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(about = "Job Shop Scheduling Problem (JSP) Solver", long_about = None)]
pub struct Cli {
    #[arg(short, long = "file", value_name = "FILE")]
    filepath: PathBuf,

    #[arg(short, long, value_name = "SECONDS")]
    timeout: Option<u64>,
}

impl Cli {
    pub fn filepath(&self) -> &Path {
        &self.filepath
    }

    pub fn timeout(&self) -> Option<Duration> {
        match self.timeout {
            Some(0) | None => None,
            Some(secs) => Some(Duration::from_secs(secs)),
        }
    }
}
