# Sanntid Group 12 V25

Project and exercies completed in the course TTK4145 Real-Time Systems (Sanntid) at NTNU.

# Install and Running Instructions 

1. Clone the repository:
    ```bash 
    git clone https://github.com/fredrswa/sanntid_project/
    ````

2. Navigate to the project directory.

3. ```bash
    # Install rust and cargo
    sudo apt-get install rustc
    sudo apt-get install cargo
    # Install xterm
    sudo apt-get install xterm
    ```


## How to run the project
**Building**

Update ```Cargo.toml``` with wanted values for num_floors, network ports, etc. Run ```cargo build --release``` and put the resulting binary in the following structure.
```
release/
├── peerX/                      # Run folder
│   ├── peer                    # binary
│   └── cab_recover.toml        # Utility functions
└── tools/                      # Dependent binaries
    ├── hall_request_assigner   
    ├── elevatorserver
    └── SimElevatorServer
```
*Using Makefile*

Commands: 
```bash
make clean      # removes builds and release folder
make            # builds 4 targets; 3 sim peers and 1 lab peer
make run_local  # runs sim peers and ./simulate_labserver
```
**Running**

```bash
./peer <id> <startupstate>
```
 - ```<id>``` is a integer from ```0 -> num_peers```. Specify each peer with a different id.
 - ```startupstate = primary/humble``` or defaults to backup. 
    - ```primary:``` use when initializing all elevators
    - ```humble:``` use when all peers are running and this peer is to rejoin a already running system.

If this is handled properly, running one of these commands will run the entire system correctly with backup states and correct recovery startup.


# Project Code Overview

This project is a peer-to-peer elevator system divided into three main modules: FSM-Module, IO-Module, and Network-Module.

The goal is to develop software that manages n elevators operating simultaneously across m floors.

Each elevator maintains a shared worldview of the system and communicates using message passing between these modules.


## FSM-Module

This module handles the logic of a single elevator, including:

Light and button operations

Door control and timers

Handling and processing elevator requests

Communicates with the IO module using channels.


## IO-Module

The IO module is responsible for:

Storing and retrieving system states.

Assigning hall requests based on the hall_request_assigner.

Communicating accepted requests to the FSM module and state updates to the network module.



## Network-Module

Handles peer-to-peer communication between elevators using UDP and JSON-based messages : udp_send and udp_receive. 

Ensures that each elevator has a synchronized worldview of the system.

It is responsible for sendign and recieving heartbeats, to detect dead elevators: send_heartbeat and receive_hearbeat.

Sends other elevator states to IO using channels

## MAIN

Run all modules

Opens channels between modules


