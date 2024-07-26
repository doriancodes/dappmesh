use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
	#[arg(value_enum)]
	pub env: DevenvAction,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DevenvAction {
	Install,
	Check,
}

#[derive(Subcommand)]
pub enum Commands {
	Resource {
		#[arg(short, long)]
		action: String,
	},
}
