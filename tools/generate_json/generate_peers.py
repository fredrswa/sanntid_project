import argparse
import socket
import json
import pandas as pd


parser = argparse.ArgumentParser(description="Creates N peer states")

parser.add_argument("N", help="Number of Peers/Elevators")

args = parser.parse_args()

print(f"Creating JSON for {args.N} peer states...")

def find_free_port():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('localhost', 0))
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        return s.getsockname()[1]


local_ip = socket.gethostbyname(socket.gethostname())

ids = [f"id:{id}" for id in range(int(args.N))]
ips = ["".join((local_ip,":", str(find_free_port()))) for _ in range(int(args.N))]


for i in range(int(args.N)):
    json_dict = {}
    for ii in range(int(args.N)):
        json_dict["id"] = f"id:{ii+1}"
        json_dict["ip"] = ips[ii]
        json_dict["peers"] = ips[:ii] + ips[ii + 1:]
        json_dict["connected"] = {}
        for iii in range(int(args.N)):
            json_dict["connected"][f"id:{iii+1}"] = False 

        with open(f"peer_state_id:{ii+1}.json", "w") as json_file:
            json.dump(json_dict, json_file, indent=4)


used_ports = pd.DataFrame({"used_ports": ips})
used_ports.to_csv("used_ports.csv", index=False, header=False)

print("Completed.")