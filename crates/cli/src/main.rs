mod cli;
mod commands;
mod shell_env;
mod setup;
mod validation;

use crate::cli::{Cli, DevenvAction};
use crate::setup::cargo_binaries;
use clap::Parser;
use std::error::Error;
use crate::validation::validate_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let cli = Cli::parse();

	match &cli.env {
		DevenvAction::Install => {
			//install().await;
			cargo_binaries().await?;
		}
		DevenvAction::Check => {

			validate_all().await?;
		}
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use assert_cmd::prelude::*;
	use predicates::prelude::predicate;
	use std::process::Command;

	#[test]
	fn test() -> Result<(), Box<dyn std::error::Error>> {
		let mut cmd = Command::cargo_bin("dappctl")?;

		cmd.arg("check");

		cmd.assert().success().stdout(predicate::str::contains("Validation"));

		Ok(())
	}
}
