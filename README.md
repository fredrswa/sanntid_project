# Sanntid Group 12 V25

Project and exercies completed in the course TTK4145 Real-Time Systems (Sanntid) at NTNU.


## Overview

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



run the following to put packetloss on all ports in use:

sudo packetloss -p 30019,30029,30039,30049,40000,40010,40005 -r 0.9

run to clear:

sudo packetloss -f
