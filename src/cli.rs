use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Blocker",
    about = "Block everything except yourself.",
    author = "Arne Beer <contact@arne.beer>"
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: SubCommand,
    pub csv_path: PathBuf,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// Show the names and essids of all available stations
    Stations,
    /// Block all devices that are connected with a specific station.
    Block {
        bssid: String,
        interface: String,
        excluded_macs: Vec<String>,
    },
}
