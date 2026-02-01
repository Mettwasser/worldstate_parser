import argparse
import json
import os
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

def download_file(file_info):
    remote_path = file_info['remote_path']
    raw_url = file_info['raw_url']
    
    parent_dir = os.path.dirname(remote_path)
    if parent_dir and not os.path.exists(parent_dir):
        os.makedirs(parent_dir, exist_ok=True)
        
    print(f"Downloading {remote_path}...")
    try:
        # Add User-Agent to avoid potential 403s on some raw endpoints or if checks are stricter
        req = urllib.request.Request(raw_url, headers={'User-Agent': 'Python-Worldstate-Parser'})
        with urllib.request.urlopen(req) as response, open(remote_path, 'wb') as out_file:
            out_file.write(response.read())
    except Exception as e:
        print(f"Failed to download {remote_path}: {e}")

def main():
    parser = argparse.ArgumentParser(description="Fetch data from GitHub")
    parser.add_argument("--owner", "-o", default="WFCD")
    parser.add_argument("--repo", "-r", default="warframe-worldstate-data")
    parser.add_argument("--branch", "-b", default="master")
    parser.add_argument("--target_folder", "-t", default="data")
    
    args = parser.parse_args()
    
    api_url = f"https://api.github.com/repos/{args.owner}/{args.repo}/git/trees/{args.branch}?recursive=1"
    
    print(f"Fetching file list from {args.repo}...")
    
    headers = {'User-Agent': 'Python-Worldstate-Parser', 'Accept': 'application/vnd.github+json'}
    req = urllib.request.Request(api_url, headers=headers)
    
    try:
        with urllib.request.urlopen(req) as response:
            tree_data = json.loads(response.read().decode())
    except Exception as e:
        print(f"Failed to fetch tree: {e}")
        return

    files_to_download = []
    # Nu script logic: $parsed_path.parent == $target_folder
    # This means it only downloads files directly inside the target folder, not subdirectories of it.
    
    for row in tree_data.get('tree', []):
        path_str = row['path']
        path_obj = Path(path_str)
        
        # Check if parent matches target_folder exactly (direct children)
        if str(path_obj.parent) == args.target_folder and path_obj.suffix == ".json" and row['type'] == "blob":
            raw_url = f"https://raw.githubusercontent.com/{args.owner}/{args.repo}/{args.branch}/{path_str}"
            files_to_download.append({
                'remote_path': path_str,
                'raw_url': raw_url
            })
            
    print(f"Found {len(files_to_download)} JSON files. Starting download...")
    
    # Use ThreadPoolExecutor to download in parallel
    with ThreadPoolExecutor(max_workers=10) as executor:
        executor.map(download_file, files_to_download)
        
    print("Download complete.")

if __name__ == "__main__":
    main()
