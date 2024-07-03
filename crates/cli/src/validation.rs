use crate::commands::{CmdCheck, CMD_CHECKS};
use crate::env::Executable;
use crate::env::ExecutableError::IOError;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::process::Output;
use std::str;
use tokio::task::JoinSet;
use colored::Colorize;

pub(crate) type CheckResult = (CmdCheck, ValidationResult);
pub(crate) type CheckInput = (CmdCheck, Output);

pub struct Validator {
	cmd_check: CmdCheck,
	output: Output,
}

impl Validator {
	pub async fn new(cmd_check: CmdCheck, output: Output) -> Self {
		Self {
			cmd_check,
			output,
		}
	}
	pub async fn validate(&self) -> CheckResult {
		let msg = str::from_utf8(&self.output.stderr).unwrap().to_string();
		if !self.output.status.success() {
			(
				self.cmd_check,
				ValidationResult::Failure {
					msg,
				},
			)
		} else {
			if msg.is_empty() {
				(self.cmd_check, ValidationResult::Success)
			} else {
				(
					self.cmd_check,
					ValidationResult::Warning {
						msg,
					},
				)
			}
		}
	}
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ValidationResult {
	Success,
	Failure {
		msg: String,
	},
	Warning {
		msg: String,
	},
}

pub async fn validate_fn(cmd_check: CmdCheck, output: Output) -> CheckResult {
	if !&output.status.success() {
		(
			cmd_check,
			ValidationResult::Failure {
				msg: str::from_utf8(&output.stderr).unwrap().to_string(),
			},
		)
	} else {
		(cmd_check, ValidationResult::Success)
	}
}

pub async fn validate_all2() {
	let checks = CMD_CHECKS.clone();

	let mut tasks:JoinSet<CheckResult> = JoinSet::new();

	for (key, value) in checks.iter() {
		let cmd_check: &CmdCheck = key;

		tasks.spawn(
			perform_check(cmd_check.clone())
		);
	}

	while let Some(res) = tasks.join_next().await {
		let (cmd_check, validation_result) = res.unwrap();
		pretty_print(cmd_check, validation_result)
	}


}

pub fn pretty_print(cmd_check: CmdCheck, validation_result: ValidationResult){
	let fmt_check = format!("{}", cmd_check);
	match validation_result {
		ValidationResult::Success => {
			println!("{} {}", "Validation for", fmt_check.bold());
			println!("Successful validations for: {}", fmt_check);
			println!("{}", "___________________________________________________________________________");
		}
		ValidationResult::Failure { .. } => {
			println!("{} {}", "Validation for", fmt_check.bold());
			println!("An {} occurred, for more info run: {}", "error".red(), CMD_CHECKS.get(&cmd_check).unwrap().blue().italic());
			println!("___________________________________________________________________________");

		}
		ValidationResult::Warning { .. } => {
			println!("{} {}", "Validation for", fmt_check.bold());
			println!("A {} occurred, for more info run: {}", "warning".yellow(), CMD_CHECKS.get(&cmd_check).unwrap().blue().italic());
			println!("___________________________________________________________________________");

		}
	}

}

pub async fn perform_check(cmd_check: CmdCheck) -> CheckResult {

	let cmd = CMD_CHECKS.get(&cmd_check).unwrap().to_string();
	let res = Executable::lazy_build(&cmd).await.exec().await.map_err(IOError).unwrap();
	let (check, result) = Validator::new(cmd_check, res).await.validate().await;


	(check, result)
}

#[cfg(test)]
mod tests {
	use crate::commands::CmdCheck;
	use crate::validation::{ValidationResult, Validator};
	use std::os::unix::process::ExitStatusExt;
	use std::process::{ExitStatus, Output};

	#[tokio::test]
	async fn test_validation_error() {
		let cmd_check = CmdCheck::AccessToGHRC;
		let output = Output {
			status: ExitStatus::from_raw(1),
			stdout: "".as_bytes().to_vec(),
			stderr: "test error".as_bytes().to_vec(),
		};

		let validator = Validator::new(cmd_check, output);

		let expected = ValidationResult::Failure {
			msg: "test error".to_string(),
		};
		let (_, validation_result) = validator.await.validate().await;
		assert_eq!(validation_result, expected);
	}

	#[tokio::test]
	async fn test_validation_success() {
		let cmd_check = CmdCheck::AccessToGHRC;
		let output = Output {
			status: ExitStatus::from_raw(0),
			stdout: "".as_bytes().to_vec(),
			stderr: "".as_bytes().to_vec(),
		};

		let validator = Validator::new(cmd_check, output);

		let expected = ValidationResult::Success;

		let (_, validation_result) = validator.await.validate().await;
		assert_eq!(validation_result, expected);
	}

	#[tokio::test]
	async fn test_validation_warning() {
		let cmd_check = CmdCheck::AccessToGHRC;
		let output = Output {
			status: ExitStatus::from_raw(0),
			stdout: "".as_bytes().to_vec(),
			stderr: "some warning".as_bytes().to_vec(),
		};

		let validator = Validator::new(cmd_check, output);

		let expected = ValidationResult::Warning {
			msg: "some warning".to_string(),
		};

		let (_, validation_result) = validator.await.validate().await;
		assert_eq!(validation_result, expected);
	}
}
