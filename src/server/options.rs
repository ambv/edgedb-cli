use std::fmt;
use std::str::FromStr;

use clap::{Clap, AppSettings};
use serde::{Serialize, Deserialize};

use crate::server::version::Version;
use crate::server::methods::InstallMethod;


#[derive(Clap, Debug, Clone)]
#[clap(setting=AppSettings::DisableVersion)]
pub struct ServerCommand {
    #[clap(subcommand)]
    pub subcommand: Command,
}

#[derive(Clap, Clone, Debug)]
pub enum Command {
    #[clap(about="Install edgedb-server")]
    Install(Install),
    #[clap(about="List available and installed versions of the server")]
    ListVersions(ListVersions),
    #[clap(about="Initialize a new server instance")]
    Init(Init),
    #[clap(about="Start an instance")]
    Start(Start),
    #[clap(about="Stop an instance")]
    Stop(Stop),
    #[clap(about="Restart an instance")]
    Restart(Restart),
    #[clap(about="Status of an instance")]
    Status(Status),
    #[clap(about="Upgrade installations and instances")]
    Upgrade(Upgrade),
    #[clap(name="_detect")]
    _Detect(Detect),
}

#[derive(Clap, Debug, Clone)]
pub struct Install {
    #[clap(short="i", long)]
    pub interactive: bool,
    #[clap(long)]
    pub nightly: bool,
    #[clap(long, conflicts_with="nightly")]
    pub version: Option<Version<String>>,
    #[clap(long, possible_values=&["package", "docker"][..])]
    pub method: Option<InstallMethod>,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting=AppSettings::Hidden)]
pub struct ListVersions {
    #[clap(long)]
    pub installed_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StartConf {
    Auto,
    Manual,
}

#[derive(Clap, Debug, Clone)]
pub struct Init {
    #[clap(about="Database server instance name", default_value="default")]
    pub name: String,
    #[clap(long)]
    pub system: bool,
    #[clap(short="i", long)]
    pub interactive: bool,
    #[clap(long)]
    pub nightly: bool,
    #[clap(long, conflicts_with="nightly")]
    pub version: Option<Version<String>>,
    #[clap(long, possible_values=&["package", "docker"][..])]
    pub method: Option<InstallMethod>,
    #[clap(long)]
    pub port: Option<u16>,
    #[clap(long, default_value="auto",
           possible_values=&["auto", "manual"][..])]
    pub start_conf: StartConf,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting=AppSettings::DisableVersion)]
pub struct Start {
    #[clap(about="Database server instance name", default_value="default")]
    pub name: String,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting=AppSettings::DisableVersion)]
pub struct Stop {
    #[clap(about="Database server instance name", default_value="default")]
    pub name: String,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting=AppSettings::DisableVersion)]
pub struct Restart {
    #[clap(about="Database server instance name", default_value="default")]
    pub name: String,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting=AppSettings::DisableVersion)]
pub struct Status {
    #[clap(about="Database server instance name", default_value="default")]
    pub name: String,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting=AppSettings::DisableVersion, after_help="\
There are few modes of operation of this command:

edgedb server upgrade
  Without arguments this command upgrades all instances which aren't running
  nightly EdgeDB to a latest minor version of the server.

edgedb <name> [--to-version=<ver>|--to-nightly]
  Upgrades specified instance to the specified major version of the server or
  to the latest nightly, by default upgrades to the latest stable. This only
  works for instances that initially aren't running nightly.

edgedb --nightly
  Upgrades all existing nightly instances to the latest EdgeDB nightly.
")]
pub struct Upgrade {
    /// Upgrade all nightly instances
    #[clap(long)]
    pub nightly: bool,

    /// Upgrade specified instance(s) to a specified major version
    #[clap(long)]
    pub to_version: Option<Version<String>>,

    /// Upgrade specifies instance to a latest nightly version
    #[clap(long)]
    pub to_nightly: bool,

    /// Only upgrade specicified database instance
    pub name: Option<String>,

    /// Verbose output
    #[clap(short="v", long)]
    pub verbose: bool,

    /// Force upgrade process even if there is no new version
    #[clap(long)]
    pub force: bool,
}

#[derive(Clap, Debug, Clone)]
#[clap(setting=AppSettings::Hidden)]
#[clap(setting=AppSettings::DisableVersion)]
pub struct Detect {
}

impl FromStr for StartConf {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<StartConf> {
        match s {
            "auto" => Ok(StartConf::Auto),
            "manual" => Ok(StartConf::Manual),
            _ => anyhow::bail!("Unsupported start configuration, \
                options: `auto`, `manual`"),
        }
    }
}

impl StartConf {
    fn as_str(&self) -> &str {
        match self {
            StartConf::Auto => "auto",
            StartConf::Manual => "manual",
        }
    }
}

impl fmt::Display for StartConf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

