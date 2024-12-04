use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use fastbloom::BloomFilter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> Result<()> {
    let args = Arguments::parse();

    match args.command {
        Command::Build {
            source,
            num_bits,
            num_items,
        } => {
            args.filter_file.mk_parent_dir()?;
            let mut filter = BloomFilter::with_num_bits(num_bits).expected_items(num_items);
            for line in source.open().lines() {
                filter.insert(&line?);
            }
            filter.save(args.filter_file).context("Saving filter")?;
        }
    }

    Ok(())
}

#[derive(Subcommand, Debug)]
enum Command {
    Build {
        /// Entries, one per line, to use when building the Bloom filter.
        #[arg()]
        source: Option<PathBuf>,

        /// The number of bits of storage for the filter:
        #[arg(long, default_value = "1024")]
        num_bits: usize,

        /// How many items are expted to be stored.
        #[arg(long, default_value = "400000000")]
        num_items: usize,
    },
}

#[derive(Parser, Debug)]
struct Arguments {
    #[command(subcommand)]
    command: Command,

    /// The location of the Bloom filter file.
    #[arg(
        short,
        long,
        env = "MOL_FILTER",
        default_value = "model/filter.mobloom"
    )]
    filter_file: PathBuf,
}

trait DefaultToStdin {
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

trait FilterStorage {
    fn save(&self, path: PathBuf) -> Result<()>;
}

impl FilterStorage for BloomFilter {
    fn save(&self, path: PathBuf) -> Result<()> {
        serde_json::to_writer(File::create(path)?, self)?;
        Ok(())
    }
}

trait DirExt {
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
