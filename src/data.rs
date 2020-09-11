use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Station {
    pub bssid: String,
    pub first_time_seen: String,
    pub last_time_seen: String,
    pub channel: String,
    pub privacy: Option<String>,
    pub cipher: Option<String>,
    pub authentication: Option<String>,
    pub power: String,
    pub beacons: String,
    pub ivs: String,
    pub lan_ip: String,
    pub essid: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct Client {
    pub station_mac: String,
    pub first_time_seen: String,
    pub last_time_seen: String,
    pub power: String,
    pub packets: String,
    pub bssid: String,
    pub probed_essids: Option<String>,
}
