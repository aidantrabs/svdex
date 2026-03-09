use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "svdex", about = "svd image compression lab")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// compress an image using truncated svd
    Compress {
        image: PathBuf,

        #[arg(short, long, default_value_t = 50)]
        k: usize,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// run compression experiments at multiple ranks
    Experiment {
        image: PathBuf,

        #[arg(short, long, value_delimiter = ',', default_values_t = vec![1, 2, 5, 10, 20, 50, 100, 150, 200])]
        ranks: Vec<usize>,
    },

    /// show image info and singular value preview
    Info {
        image: PathBuf,
    },
}
