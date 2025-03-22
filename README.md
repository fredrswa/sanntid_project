# Sanntid Group 12 V25

Project and exercies completed in the course TTK4145 Real-Time Systems (Sanntid) at NTNU.

---

## Plan
0. Running:
    - Schematics:
        - Create schematics for each module, peer, network
    - Project:
        - Keep things for project report.
    - Get really good at rust
1. Modules:
    - Currently unknown place:
        - Take orders from assigner and create state for other peers, and create orders for this peer.
    - Testing module:
        - Automate testing environment?? Use test module for testing, only function calls in main.
    - Assigner: 
        - [repo](https://github.com/TTK4145/Project-resources)
        - Communicates with IO and network.
    - Network:
        - Inputs (JSON[state], heartbeat)
        - Function:
          -  Rx/Tx states
          -  Rx/Tx heartbeat
          -  Heartbeat procedure (what?)
        - Outputs (JSON[state], heartbeat)
        -  
        - Create test script
        - Figure out a package that contains info. (or use more threads).
        - What type of info do we want the network module to receive
    - IO/Server
        - Input(Elevator Console)
        - Function:
            - Launch and port assignment
            - Communicate with more than one elevator (TCP handler)
            - Heartbeat
            - Method for relaunching
        - Outputs (Orders)
    - FSM/IO:
        - Inputs (Orders)
        - Function:
            - Operate elevator
              - Elevator logic
        - Outputs (Elevator/Light instructions)
        - 
        - Reconsider default statement in main loop
        - Maybe be more consistent with forcing u8, lots of recasting.
        - Modularise single elevator into modules IO and FSM in peer module.
    - Brain/Coordinator
      - Could be worth a read: [Message passing between threads](https://doc.rust-lang.org/stable/book/ch16-02-message-passing.html)
      - Inputs (?)
      - Function:
        - Run on startup
        - Initalizes all peer modules
        - Connect to IO/Server
        - "Route" traffic between modules
        - 
      - Outputs (?)
2. main.rs:
    - A way too coordinate modules
    - Open channels between them


## Overview

This project is a peer-to-peer elevator system divided into three main modules: FSM-Module, IO-Module, and Network-Module.

The goal is to develop software that manages n elevators operating simultaneously across m floors.

Each elevator maintains a shared worldview of the system and communicates using message passing between these modules.


## FSM-Module

This module handles the logic of a single elevator, including:

Light and button operations

Door control and timers

Handling and processing elevator requests

Communicates with the IO module using channels: fsm_to_io_tx and call_from_io_rx.


## IO-Module

The IO module is responsible for:

Managing all input operations.

Storing and retrieving system states.

Assigning hall requests based on the hall_request_assigner.

Communicating accepted requests to the FSM module and state updates with the network module using: call_button_from_io_tx, network_to_io_rx, io_to_network_tx and fsm_to_io_rx.



## Network-Module

Handles peer-to-peer communication between elevators using UDP and JSON-based messages : udp_send and udp_receive. 

Ensures that each elevator has a synchronized worldview of the system.

It is responsible for sendign and recieving heartbeats, to detect dead elevators: send_heartbeat and receive_hearbeat.

## MAIN

Run all modules

Opens channels between modules


## Install instructions 

1. Clone the repository:
    git clone <repository-url>

2. Navigate to the project directory.

3. Install dependencies:
    cargo build

4. Ensure that you have Rust and Cargo installed.


## How to run the project
1. Build the project:
    cargo build

2. Run the project:
    cargo run 

## Configurations
