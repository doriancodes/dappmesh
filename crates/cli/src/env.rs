use std::ffi::OsStr;
use std::io::Error;
use std::process::{Command, Output};

pub struct Executable {
	pub command: Command,
}

impl Executable {
	pub async fn exec(&mut self) -> Result<Output, Error> {
		self.command.output()
	}

	pub async fn lazy_build(g: impl AsRef<OsStr>) -> Self {
		let mut cmd = Command::new("sh");
		cmd.arg("-c").arg(&g);
		Self {
			command: cmd,
		}
	}
}

#[derive(Debug)]
pub enum ExecutableError {
	IOError(Error),
}
