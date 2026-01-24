def main [
    --data-dir (-d): path = "./data"
    --out-dir (-o): path = "./assets"
    --file-name (-n): path = "relays.json"
] {
    let source_file = $data_dir | path join "solNodes.json"
    
    let hubs = open $source_file
        | transpose key val
        | where key ends-with "HUB"
        | update val { |row|
            $row.val.value 
            | parse "{name} {relay} ({_})"
            | get 0                          # Select the first match to get a record, not a list
            | $"($in.name) ($in.relay)"      # Interpolate simple strings
        }
        | transpose -r -d                    # Restore original dictionary structure
        
    # Ensure output directory exists and save
    mkdir $out_dir
    $hubs | save -f ($out_dir | path join $file_name)
}