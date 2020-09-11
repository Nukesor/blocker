use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;

use anyhow::Result;
use csv::Reader;
use lazy_static::lazy_static;

use crate::data::{Client, Station};

lazy_static! {
    static ref REPLACE_MAP: HashMap<&'static str, &'static str> = [
        ("BSSID", "bssid"),
        ("First time seen", "first_time_seen"),
        ("Last time seen", "last_time_seen"),
        ("channel", "channel"),
        ("Speed", "speed"),
        ("Privacy", "privacy"),
        ("Cipher", "cipher"),
        ("Authentication", "authentication"),
        ("Power", "power"),
        ("# beacons", "beacons"),
        ("# IV", "ivs"),
        ("LAN IP", "lan_ip"),
        ("ID-length", "id_length"),
        ("ESSID", "essid"),
        ("Key", "key"),
        ("Station MAC", "station_mac"),
        ("# packets", "packets"),
        ("Probed ESSIDs", "probed_essids"),
        ("\r", ""),
        (", ", ","),
    ]
    .iter()
    .copied()
    .collect();
}

pub fn get_csv_data(csv_path: &PathBuf) -> Result<(Vec<Station>, Vec<Client>)> {
    let mut csv_content = std::fs::read_to_string(csv_path)?;

    for (search, replace) in REPLACE_MAP.iter() {
        csv_content = csv_content.replace(search, replace);
    }

    let mut splitted: Vec<&str> = csv_content.split("\n\n").collect();
    let _ = splitted.pop();
    let raw_clients = splitted.pop().unwrap();
    let raw_stations = splitted.pop().unwrap();

    let mut stations = Vec::new();
    let mut station_reader = Reader::from_reader(Cursor::new(raw_stations));
    for station_result in station_reader.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let station: Station = station_result?;
        stations.push(station);
    }

    let mut clients = Vec::new();
    let mut client_reader = Reader::from_reader(Cursor::new(raw_clients));
    for client_result in client_reader.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let client: Client = client_result?;
        clients.push(client);
    }
    Ok((stations, clients))
}
