use anyhow::{Context, Result};
use fastbloom::BloomFilter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub trait DefaultToStdin {
    fn open(&self) -> Box<dyn BufRead>;
}

impl DefaultToStdin for Option<PathBuf> {
    fn open(&self) -> Box<dyn BufRead> {
        match self {
            None => Box::new(BufReader::new(std::io::stdin())),
            Some(filename) => {
                let file = File::open(filename).expect("Cannot open source file");
                Box::new(BufReader::new(file))
            }
        }
    }
}

pub trait FilterStorage {
    fn save(&self, path: PathBuf) -> Result<()>;
    fn load(path: PathBuf) -> Result<Box<Self>>;
}

impl FilterStorage for BloomFilter {
    fn save(&self, path: PathBuf) -> Result<()> {
        serde_json::to_writer(File::create(path)?, self)?;
        Ok(())
    }

    fn load(path: PathBuf) -> Result<Box<Self>> {
        let file = File::open(path).context("Reading filter")?;
        let filter: BloomFilter =
            serde_json::from_reader(file).context("Decoding filter file content")?;
        Ok(Box::new(filter))
    }
}

pub trait DirExt {
    fn mk_parent_dir(&self) -> Result<()>;
}

impl DirExt for PathBuf {
    fn mk_parent_dir(&self) -> Result<()> {
        match self.parent() {
            None => Ok(()), // Top of the file system, nothing to do
            Some(p) if std::fs::exists(p).unwrap_or(false) => Ok(()), // Parent exists, nothing to do
            Some(p) => std::fs::create_dir_all(p).context("Creating output path"),
        }
    }
}
