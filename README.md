# Sanntid Group 12 V25

Project and exercies completed in the course TTK4145 Real-Time Systems (Sanntid) at NTNU.

# Install and Running Instructions 

1. Clone the repository:
    ```bash 
    git clone https://github.com/fredrswa/sanntid_project/
    ````

2. Navigate to the project directory.

3. Install dependencies:
    ```bash
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
**Using Makefile**

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
 - ```<id>``` is a integer from ```0 <= id < num_peers```. Specify each peer with a different id.
 - ```<startupstate> = primary/humble``` or defaults to backup. 
    - ```primary:``` use when initializing all elevators
    - ```humble:``` use when peers are running and this peer is to rejoin a already running system. Will safely rejoin the system without overwriting good information.

If this is handled properly, running one of these commands will run the entire system correctly with backup states and correct recovery startup.


# Project Code Overview

This project is a peer-to-peer elevator system divided into three main modules: FSM-Module, IO-Module, and Network-Module.

The goal is to develop software that manages n elevators operating simultaneously across m floors.

Each elevator maintains a shared worldview of the system and communicates using message passing between these modules.

**Structure**

We have based to code structure on Rust's module implementation. With modules as folders with mod.rs within them.
```
peer/
├── src/                     
│   ├── mod_hardware/
│   │   └── mod.rs
│   │
│   ├── mod_backup/
│   │   └── mod.rs
│   │
│   ├── mod_fsm/
│   │   ├── mod.rs
│   │   ├── fsm.rs
│   │   ├── timer.rs
│   │   └── requests.rs
│   │
│   ├── mod_io/
│   │   ├── mod.rs
│   │   └── io.rs
│   │
│   ├── mod_network/ 
│   │   ├── mod.rs
│   │   └── network.rs   
│   │
│   ├── main.rs
│   ├── config.rs          
│   └── lib.rs    
│
├── cab_recover.toml
├── Config.toml
└── Cargo.toml
```


### Main

 1. Check/spawn hardware
 2. Setup channels between modules
 3. Spawn modules in their respective state
 4. Loop for fault tolerance

### FSM-Module

This module handles the logic of a single elevator, including:
 - Light and button operations
 - Door control and timers
 - Handling and processing elevator requests
 - Communicates with the IO module using channels.


### IO-Module

The IO module is responsible for:
 - Storing and retrieving system states.
 - Assigning hall requests based on the hall_request_assigner.
 - Communicating accepted requests to the FSM module and state updates to the network module.



### Network-Module
Sends other elevator states to IO using channels
Handles peer-to-peer networking using ```udp```:
- Ensure that each peer has a synchronized view of it's peers
- Sending and receiving heartbeats to monitor peer health.
- Uses serde_json to serialize and deserialize messages.
### Backup-Module
Reponsible for the startup states and recovery of orders
- Ensures initalizing a system is done well
- Ensures rejoining peer is done well
- Ensures a crashing program can restart itself


