mod cli;
mod commands;
mod env;
mod validation;

use crate::cli::{Cli, EnvAction};
use crate::validation::validate_all2;
use clap::Parser;
use std::process::Command;

#[tokio::main]
async fn main() {
	let cli = Cli::parse();

	match &cli.env {
		EnvAction::Setup => {
			println!("setup");
		}
		EnvAction::Check =>  {

			validate_all2().await
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use assert_cmd::prelude::*;
	use predicates::prelude::predicate;

	#[test]
	fn test() -> Result<(), Box<dyn std::error::Error>> {
		let mut cmd = Command::cargo_bin("dapp-cli")?;

		cmd.arg("check");

		cmd.assert().success().stdout(predicate::str::contains("Validation"));

		Ok(())
	}
}
