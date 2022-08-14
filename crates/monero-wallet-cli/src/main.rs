use std::path::PathBuf;

use anyhow::anyhow;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Name of wallet file
    #[clap(short, long)]
    path: PathBuf,

    /// Password of wallet file
    #[clap(short = 'P', long)]
    password: Option<String>,

    /// Language to be used to generate electrum seed mnemonic
    #[clap(short, long, default_value = "english")]
    language: String,

    /// Number of rounds for key derivation function
    #[clap(short, long, default_value = "1")]
    kdf_rounds: u64,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let password = match cli.password {
        Some(password) => password,
        None => rpassword::prompt_password("Wallet password: ")?,
    };
    let w = monero_sys::Wallet::new(
        cli.path.to_str().ok_or(anyhow!("Invalid path"))?,
        &password,
        &cli.language,
        monero_sys::NetworkType::MAINNET,
        cli.kdf_rounds,
    )?;

    Ok(())
}
