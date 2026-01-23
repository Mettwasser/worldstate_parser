const url = "https://drops.warframestat.us/data/all.json"

mkdir drops
http get $url | save drops/data.json --force