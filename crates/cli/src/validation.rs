use std::error::Error;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::process::Output;
use colored::Colorize;
use tokio::task::JoinSet;
use crate::commands::{CMD_OPS, CmdOp};
use crate::shell_env::{EnvironmentContext};
use std::str;


pub async fn validate_all() -> Result<(), Box<dyn Error>> {
    let cmd_ops = &CMD_OPS;

    let mut final_tasks: JoinSet<String> = JoinSet::new();


    for (key, _) in cmd_ops.iter() {

        final_tasks.spawn(
            perform_check(**key)
        );
    }

    while let Some(res) = final_tasks.join_next().await {
        let result = res.map_err(|e| format!("{}", e)).unwrap();
        println!("{}", result)
    }

    Ok(())
}

pub async fn perform_check(cmd_check: CmdOp) -> String {

    let mut cmd = &mut EnvironmentContext::new().command;

    let (cmd_arg, _) = CMD_OPS[&cmd_check];
    cmd.arg(cmd_arg);

    let res = cmd.exec().await.map_err(|e| format!("{}", e)).unwrap();
    let (_, result) = Validator::new(cmd_check, res.inner.clone()).await.validate().await;

    format_out(cmd_check, result, cmd_arg)

}

pub fn format_out<C>(cmd_check: C, validation_result: ValidationResult, check: &'static str) -> String
    where
        C: Eq + PartialEq + Hash + Debug + Copy + Clone + Display + Send + Sync + 'static,
{
    let fmt_check = format!("{}", cmd_check);
    let line_separator = "___________________________________________________________________________";
    match validation_result {
        ValidationResult::Success => {
            format!("Validation for {}\n{}: {}\n{}\n", fmt_check.bold(), "Successful validation for".green(),  fmt_check.bold().green(), &line_separator)
        }
        ValidationResult::Failure {
            msg
        } => {
            format!("Validation for {}\n{}\n{}: {}\n{}\n", fmt_check.bold(), &msg.red(),"For more info run:",  check.blue().italic(), &line_separator)
        }
        ValidationResult::Warning {
            msg
        } => {
            format!("Validation for {}\n{}\n{}: {}\n{}\n", fmt_check.bold(), &msg.yellow(),"For more info run:",  check.blue().italic(), &line_separator)
        }

    }
}


pub struct Validator {
    cmd_op: CmdOp,
    output: Output,
}

impl Validator {
    pub async fn new(cmd_op: CmdOp, output: Output) -> Self {
        Self {
            cmd_op,
            output,
        }
    }
    pub async fn validate(&self) -> (CmdOp, ValidationResult) {
        let msg = str::from_utf8(&self.output.stderr).unwrap().to_string();
        if !self.output.status.success() {
            (
                self.cmd_op,
                ValidationResult::Failure {
                    msg,
                },
            )
        } else {
            if msg.is_empty() {
                (self.cmd_op, ValidationResult::Success)
            } else {
                (
                    self.cmd_op,
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


#[cfg(test)]
mod tests {
    //idea
    //https://rust-lang.github.io/api-guidelines/interoperability.html#generic-readerwriter-functions-take-r-read-and-w-write-by-value-c-rw-value
    use crate::commands::CmdOp;
    use crate::validation::{ValidationResult, Validator};
    use std::os::unix::process::ExitStatusExt;
    use std::process::{ExitStatus, Output};

    #[tokio::test]
    async fn test_validation_error() {
        let cmd_check = CmdOp::AccessToGHRC;
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
        let cmd_check = CmdOp::AccessToGHRC;
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
        let cmd_check = CmdOp::AccessToGHRC;
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
