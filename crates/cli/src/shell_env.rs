use std::borrow::Cow;
use std::ffi::OsStr;
use std::io::Error;
use std::ops::{Deref, DerefMut};
use std::process::{Command, Output};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct ExecCommand{
	command: Command,
}

pub struct DappOutput {
	pub inner: Output,
}

impl From<Command> for ExecCommand {
	fn from(value: Command) -> Self {
		Self {
			command: value,
		}
	}
}

impl From<&mut Command> for ExecCommand {
	fn from(value: &mut Command) -> Self {
		let val:&OsStr = value.get_program();
		Self {
			command: Command::new(val),
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

/*fn exec_multi_plat() -> &'static mut Command {
	if cfg!(target_os = "windows") {
		Command::new("cmd").arg("/C")
	} else {
		Command::new("sh").arg("-c")
	}
}*/

impl ExecCommand {

	pub fn get_cmd(&self) -> &Command {
		&self.command
	}

	pub fn get_cmd_as_mut(&mut self) -> &mut Command {
		&mut self.command
	}

	pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
		let mut cmd = Command::new("sh");
		cmd.arg("-c").arg(program);
		Self {
			command: cmd,
		}
	}

	pub fn to_command(&self) -> Command {
		let prog = &self.command.get_program();
		let mut cmd = Command::new("sh");
		cmd.arg("-c").arg(*prog);
		cmd
	}

	pub async fn exec(&mut self) -> Result<DappOutput, Error> {
		match self.command.output() {
			Ok(out) => Ok(DappOutput {
				inner: out,
			}),
			Err(err) => Err(err),
		}
	}

	pub async fn exec_2(&mut self) -> Result<DappOutput, Error> {
		match self.command.output() {
			Ok(out) => Ok(DappOutput {
				inner: out,
			}),
			Err(err) => Err(err),
		}
	}
}

impl Into<Command> for ExecCommand {
	fn into(self) -> Command {
		self.to_command()
	}
}

trait ShellEnv {
	fn init() -> ExecCommand;
}

struct WindowsShell {
	command: ExecCommand
}

impl ShellEnv for WindowsShell {
	fn init() -> ExecCommand {
		let mut window_cmd = Command::new("cmd");
		window_cmd.arg("/C");

		ExecCommand {
			command: window_cmd
		}
		//window_cmd

	}
}

struct UnixShell {
	command: ExecCommand
}

impl ShellEnv for UnixShell {
	fn init() -> ExecCommand {
		let mut unix_cmd = Command::new("sh");
		unix_cmd.arg("-c");

		ExecCommand {
			command: unix_cmd
		}
	}
}

pub struct EnvironmentContext {
	pub command: ExecCommand
}

impl EnvironmentContext {
	pub fn new() -> EnvironmentContext {
		let cmd = if cfg!(target_os = "windows") {
			WindowsShell::init()
		} else {
			UnixShell::init()
		};

		EnvironmentContext {
			command: cmd
		}
	}
}

impl Deref for ExecCommand {
	type Target = Command;

	fn deref(&self) -> &Self::Target {
		&self.command
	}
}

impl DerefMut for ExecCommand {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.command
	}
}

#[cfg(test)]
mod tests {
	use crate::shell_env::ExecCommand;

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
