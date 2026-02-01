import subprocess
import sys

def main():
    print("Running scripts/fetch_data.py...")
    subprocess.run([sys.executable, "scripts/fetch_data.py"], check=True)
    
    print("Running scripts/fetch_drops.py...")
    subprocess.run([sys.executable, "scripts/fetch_drops.py"], check=True)

if __name__ == "__main__":
    main()
