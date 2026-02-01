import os
import urllib.request
import shutil

def main():
    url = "https://drops.warframestat.us/data/all.json"
    output_dir = "drops"
    output_file = os.path.join(output_dir, "data.json")
    
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
        
    print(f"Downloading {url} to {output_file}...")
    
    headers = {'User-Agent': 'Python-Worldstate-Parser'}
    req = urllib.request.Request(url, headers=headers)
    
    try:
        with urllib.request.urlopen(req) as response, open(output_file, 'wb') as out_file:
            shutil.copyfileobj(response, out_file)
        print("Download complete.")
    except Exception as e:
        print(f"Failed to download drops: {e}")

if __name__ == "__main__":
    main()
