use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use strum_macros::EnumString;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, EnumString, strum_macros::Display)]
pub enum CmdCheck {
	AccessToGHRC,
	Workspace,
	UnitTests,
	IntegrationTests,
	FullBuild,
	BuildAndPushImg,
	CleanBuild,
	Quick,
	Devspace,
	Minikube,
}

const ACCESS_TO_GHRC_CMD: &'static str =
	"echo $GHRC_TOKEN | docker login ghcr.io -u $(git config --global user.name) --password-stdin";
const MAKE_CHECK_CMD: &'static str = "make check";
const MAKE_QUICK_CMD: &'static str = "make quick";
const MAKE_TEST_CMD: &'static str = "make test";
const MAKE_BUILD_CMD: &'static str = "make build";
const MAKE_DOCKER_CMD: &'static str = "make docker";
const MAKE_CLEAN_CMD: &'static str = "make clean";
const DEVSPACE_CMD: &'static str = "devspace dev";
const MINIKUBE_STATUS: &'static str = "minikube status";
const MAKE_INTEGRATION_TEST_CMD: &'static str = "make integration-test";

lazy_static! {
	pub static ref CMD_CHECKS: HashMap<&'static CmdCheck, &'static str> = hashmap! {
		&CmdCheck::AccessToGHRC => ACCESS_TO_GHRC_CMD,
		&CmdCheck::Workspace => MAKE_CHECK_CMD,
		&CmdCheck::Quick => MAKE_QUICK_CMD,
		&CmdCheck::UnitTests => MAKE_TEST_CMD,
	//	&CmdCheck::IntegrationTests => MAKE_INTEGRATION_TEST_CMD,
	//	&CmdCheck::FullBuild => MAKE_BUILD_CMD,
	//	&CmdCheck::BuildAndPushImg => MAKE_DOCKER_CMD,
	//	&CmdCheck::CleanBuild => MAKE_CLEAN_CMD,
	//	&CmdCheck::Devspace => DEVSPACE_CMD,
		&CmdCheck::Minikube => MINIKUBE_STATUS
	};
}
