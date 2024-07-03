use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
	#[arg(value_enum)]
	pub env: EnvAction,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum EnvAction {
	Setup,
	Check,
}

#[derive(Subcommand)]
pub enum Commands {
	Resource {
		#[arg(short, long)]
		action: String,
	},
}
