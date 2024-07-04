use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::io::{Error, Read};
use std::ops::Deref;
use std::process::{Command, Output};
use clap::error::ErrorFormatter;

#[derive(Debug)]
pub struct ExecCommand {
	command: Command,
}

impl From<Command> for ExecCommand {
	fn from(value: Command) -> Self {
		Self {
			command: value
		}
	}
}

impl From<&str> for ExecCommand {
	fn from(value: &str) -> Self {
		let mut cmd = Command::new("sh");
		cmd.arg("-c").arg(&value);
		Self {
			command: cmd,
		}
	}
}

impl ExecCommand {

	pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
		let mut cmd = Command::new("sh");
		cmd.arg("-c").arg(program);
		Self {
			command: cmd
		}
	}

	pub fn to_command(&self) -> Command {
		let prog = &self.command.get_program();
		let mut cmd = Command::new("sh");
		cmd.arg("-c").arg(*prog);
		cmd
	}

	pub async fn exec(&mut self) -> Result<Output, Error> {
		self.command.output()
	}
}

#[derive(Debug)]
pub enum ExecutableError<'a> {
	IOError {
		msg: &'a str
	},
}

/*impl Display for ExecutableError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "IO Error")
	}
}*/

/*impl std::error::Error for ExecutableError {

}*/

impl Into<Command> for ExecCommand {
	fn into(self) -> Command {
		self.to_command()
	}
}

/*impl Drop for ExecCommand {
	fn drop(&mut self) {
		write!("{}", "{}", ExecutableError::IOError {msg:""}.to_string())
	}
}*/

#[cfg(test)]
mod tests {
	use crate::env::ExecCommand;

	//https://rust-lang.github.io/api-guidelines/interoperability.html#types-are-send-and-sync-where-possible-c-send-sync
	#[test]
	fn test_send() {
		fn assert_send<T: Send>() {}
		assert_send::<ExecCommand>();
	}

	#[test]
	fn test_sync() {
		fn assert_sync<T: Sync>() {}
		assert_sync::<ExecCommand>();
	}

}