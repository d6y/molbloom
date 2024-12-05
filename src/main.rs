use anyhow::{Context, Result};
use clap::Parser;
use fastbloom::BloomFilter;
use std::io::BufRead;

mod args;
mod io;

use io::{DefaultToStdin, DirExt, FilterStorage};

use args::{Arguments, Command};

fn main() -> Result<()> {
    let args = Arguments::parse();

    match args.command {
        Command::Build {
            source,
            num_bits,
            num_items,
            fpr,
        } => {
            args.filter_file.mk_parent_dir()?;

            let mut filter = if let Some(rate) = fpr {
                BloomFilter::with_false_pos(rate).expected_items(num_items)
            } else {
                BloomFilter::with_num_bits(num_bits).expected_items(num_items)
            };

            for line in source.open().lines() {
                filter.insert(&line?);
            }
            filter.save(args.filter_file).context("Saving filter")?;
        }

        Command::Query { source } => {
            let filter = BloomFilter::load(args.filter_file)?;
            for line in source.open().lines() {
                println!("{}", filter.contains(&line?));
            }
        }
    }

    Ok(())
}
