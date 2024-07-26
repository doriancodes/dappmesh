use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::process::Command;
use crate::commands::{CargoOp, CmdOp, CARGO_OPS, CMD_OPS, Exec};
use crate::shell_env::ExecCommand;
use crate::validation::{perform_check, pretty_print, ValidationResult, Validator};
use anyhow::Result;
use cargo_run_bin::{binary, metadata};
use tokio::task::JoinSet;

pub async fn install_all() -> std::result::Result<(), Box<dyn Error>> {
	let cmd_ops = &CMD_OPS;

	let mut final_tasks: JoinSet<()> = JoinSet::new();

	final_tasks.spawn(gen_tasks(cmd_ops));

	while let Some(res) = final_tasks.join_next().await {
		continue;
	}

	Ok(())

}

pub async fn func1() {
	let status = Command::new("docker").arg("--help").status().unwrap();

	println!("{}", status.code().unwrap())


}

async fn gen_tasks(op_map: &HashMap<&CmdOp, Exec>)
{
	let mut tasks: JoinSet<()> = JoinSet::new();

	for (key, _) in op_map.iter() {
		let (check, _) = *op_map.get(key).unwrap();

		tasks.spawn(perform_check(**key));
	}

	while let Some(res) = tasks.join_next().await {
		continue
	}
}
pub async fn cargo_binaries() -> Result<()> {
	let packages = metadata::get_binary_packages()?;

	for p in packages.iter() {
		binary::install(p.clone())?;
	}

	Ok(())
}
