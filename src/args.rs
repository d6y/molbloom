use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,

    /// The location of the Bloom filter file.
    #[arg(
        short,
        long,
        env = "MOL_FILTER",
        default_value = "model/filter.mobloom"
    )]
    pub filter_file: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Command {
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

    Query {
        /// Entries to evaluate, one per line.
        #[arg()]
        source: Option<PathBuf>,
    },
}