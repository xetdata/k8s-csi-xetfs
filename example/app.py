import argparse
import os
import sys
import time

def get_dir_arg() -> str:
    parser = argparse.ArgumentParser(description='Get the name of the mounted directory to traverse')
    parser.add_argument('dir', type=str, help='directory to traverse')
    args = parser.parse_args()
    return args.dir

def byte_count_file(filepath: str) -> int:
    count = 0
    with open(filepath, 'rb') as f:
        content = f.read()
        for _ in content:
            count += 1
    print(f"done {filepath}")
    return count

def byte_count_recursive(dir: str) -> int:
    count = 0
    for root, dirs, files in os.walk(dir):
        for file in files:
            count += byte_count_file(os.path.join(root, file))
        for directory in dirs:
            count += byte_count_recursive(os.path.join(root, directory))
    print(f"done {dir}")
    return count

def main():
    dir_name = get_dir_arg()

    if not os.path.exists(dir_name):
        print("dir does not exist, exiting")
        sys.exit(1)

    count = byte_count_recursive(dir_name)
    print(f"count {count}")

    # keep container up
    time.sleep(100000000)


if __name__ == '__main__':
    main()
