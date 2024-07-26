use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::hash::Hash;
use strum_macros::EnumString;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, EnumString, strum_macros::Display)]
pub enum CmdOp {
	AccessToGHRC,
	DockerInstalled,
	BuildxInstalled,
	BubblewrapInstalled,
	KubernetesRunning,
	DockerRunning,
	KubectlInstalled,
	CargoMakeInstalled,
	CargoVetInstalled,
	CargoACLInstalled,
CargoDenyInstalled
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, EnumString, strum_macros::Display)]
pub enum CargoOp {
	CargoMakeInstalled,
	CargoVetInstalled,
	CargoACLInstalled,
	CargoDenyInstalled,

}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, EnumString, strum_macros::Display)]
pub enum CmdInstall {
	CargoDeps,
}

const ACCESS_TO_GHRC_CMD: &'static str =
	"echo $GHRC_TOKEN | docker login ghcr.io -u $(git config --global user.name) --password-stdin";

pub(crate) type Check = &'static str;
pub(crate) type Suggestion = &'static str;
pub(crate) type Exec = (Check, Suggestion);

lazy_static! {
	//cargo install --list | awk '/^\\w/ {print $1}'
	//TODO add cargo binstall and cargo-run-bin
	pub static ref CMD_OPS: HashMap<&'static CmdOp, Exec> = hashmap! {
		&CmdOp::BubblewrapInstalled => ("apt list | grep bubblewrap","sudo apt install -y bubblewrap"), //probably not needed
		//&CmdOp::AccessToGHRC => (ACCESS_TO_GHRC_CMD, "make sure to export your token: 'export GHRC_TOKEN=YOUR_TOKEN'"),
		&CmdOp::DockerInstalled => ("docker --help", "install docker"),
		&CmdOp::BuildxInstalled => ("docker buildx --help", "install buildx"),
		&CmdOp::DockerRunning => ("docker ps", "start docker"),//TODO change
		&CmdOp::KubectlInstalled => ("kubectl --help", "install kubernetes"),
		&CmdOp::KubernetesRunning => ("kubectl get nodes", "start kubernetes"),
		&CmdOp::CargoMakeInstalled => ("find \"$PWD/.bin\" -name \"cargo-make\"", "run 'dappctl install --cargo'"),
		&CmdOp::CargoVetInstalled => ("find \"$PWD/.bin\" -name \"cargo-vet\"", "run 'dappctl install --cargo'"),
		&CmdOp::CargoACLInstalled => ("find \"$PWD/.bin\" -name \"cargo-acl\"","run 'dappctl install --cargo'"),
		&CmdOp::CargoDenyInstalled => ("find \"$PWD/.bin\" -name \"cargo-deny\"", "run 'dappctl install --cargo'"),

	};

	pub static ref CARGO_OPS: HashMap<&'static CargoOp, Exec> = hashmap! {
		//find and dir commands https://crates.io/crates/coreutils
		&CargoOp::CargoMakeInstalled => ("find \"$PWD/.bin\" -name \"cargo-make\"", "run 'dappctl install --cargo'"),
		&CargoOp::CargoVetInstalled => ("find \"$PWD/.bin\" -name \"cargo-vet\"", "run 'dappctl install --cargo'"),
		&CargoOp::CargoACLInstalled => ("find \"$PWD/.bin\" -name \"cargo-acl\"","run 'dappctl install --cargo'"),
		&CargoOp::CargoDenyInstalled => ("find \"$PWD/.bin\" -name \"cargo-deny\"", "run 'dappctl install --cargo'"),
	};

}
