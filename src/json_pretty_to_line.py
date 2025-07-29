import glob
import json
import os
import argparse

def read_json_files(directory):
    # Get all JSON files in the directory
    json_files = glob.glob(os.path.join(directory, "*.json"))
    all_logs = []

    for file in json_files:
        with open(file, 'r') as f:
            try:
                logs = json.load(f)
                if isinstance(logs, list):
                    all_logs.extend(logs)
                elif isinstance(logs, dict):
                    all_logs.append(logs)
            except json.JSONDecodeError as e:
                print(f"Error reading {file}: {e}")

    return all_logs

def print_logs_per_line(logs):
    for log in logs:
        print(json.dumps(log))

def main():
    parser = argparse.ArgumentParser(description="""Read and concatenate JSON log files. 
        This will also work with pretty printed Json logs.
        Pass a directory to the script and it will read in
        and convert any file with a '.json' extension.""")
    parser.add_argument('directory', type=str, help="The directory containing JSON log files wisth a '.json' extension.")

    args = parser.parse_args()

    # Read and concatenate logs from JSON files
    logs = read_json_files(args.directory)

    # Print each log as a single line
    print_logs_per_line(logs)

if __name__ == "__main__":
    main()

