import argparse
import socket
import json
import pandas as pd

parser = argparse.ArgumentParser(description="Creates N config files")

parser.add_argument("N", help="Number of Peers/Elevators")

NUM_FLOORS = 4
NUM_BUTTONS = 3
DOOR_OPEN_S = 3

args = parser.parse_args()

print(f"Creating JSON config for {args.N} peers...")

used_ports = pd.read_csv("used_ports.csv")

def find_free_port():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('localhost', 0))
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

        if s.getsockname()[1] in used_ports:
            return find_free_port()
        return s.getsockname()[1]

local_ip = socket.gethostbyname(socket.gethostname())

udp_socket_addr = ["".join(("0.0.0.0:", str(find_free_port()))) for _ in range(int(args.N))]


for i in range(int(args.N)):
    json_dict = {}
    json_dict["num_floors"] = NUM_FLOORS
    json_dict["num_buttons"] = NUM_BUTTONS
    json_dict["num_elevators"] = int(args.N)
    json_dict["door_open_s"] = DOOR_OPEN_S
    json_dict["id"] = f"id:{i+1}"
    json_dict["elev_addr"] = "".join((local_ip,":", str(find_free_port())))
    json_dict["udp_socket_addr"] = udp_socket_addr[i]
    json_dict["udp_others_addr"] = udp_socket_addr[:i] + udp_socket_addr[i + 1:]
    json_dict["udp_recv_port"] = "placeholder"

    with open(f"config_id:{i+1}.json", "w") as json_file:
        json.dump(json_dict, json_file, indent=4)

print("Completed.")