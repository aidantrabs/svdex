use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "svdex", about = "SVD Image Compression Lab")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Compress an image using truncated SVD
    Compress {
        /// Path to the input image
        image: PathBuf,

        /// Number of singular values to keep
        #[arg(short, long, default_value_t = 50)]
        k: usize,

        /// Output path (default: output/compressed/compressed_k{k}.png)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Run compression experiments at multiple ranks
    Experiment {
        /// Path to the input image
        image: PathBuf,

        /// Comma-separated list of ranks to test
        #[arg(short, long, value_delimiter = ',', default_values_t = vec![1, 2, 5, 10, 20, 50, 100, 150, 200])]
        ranks: Vec<usize>,
    },

    /// Show image info and singular value preview
    Info {
        /// Path to the input image
        image: PathBuf,
    },
}
