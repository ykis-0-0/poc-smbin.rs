use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  /// What to do
  #[command(subcommand)]
  pub opcode: OpCode,

  /// Name for new binary
  #[arg(short = 'n', long = "name")]
  pub name: Option<String>,

  #[arg(short, long = "debug", action = ArgAction::Count)]
  verbosity: u8,

  #[arg(short, long)]
  pub quiet: bool,
}

#[non_exhaustive]
#[derive(Subcommand)]
pub enum OpCode {
  /// Retrieve content of payload
  Get,
  /// Get size of payload
  Measure,
  /// Add new payload
  Add,
  /// Drop all payload
  Clear,
  /// Drop all payload, and Add a new one
  New,
}

impl Cli {
  pub fn get_verbosity(&self) -> Result<simplelog::LevelFilter, eyre::Error> {
    use simplelog::LevelFilter;

    let rtv = match self.verbosity {
      0 => LevelFilter::Info,
      1 => LevelFilter::Debug,
      2 => LevelFilter::Trace,
      _ => eyre::bail!("There's only so much I can tell."),
    };

    Ok(rtv)
  }
}
