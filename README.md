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
    - Testing module:
        - Automate testing environment??
    - Assigner: 
        - Inputs (new_order, JSON[state] for each elevator)
        - Function: 
            - Calculate which elevator takes an order
        - Outputs (take_order: bool)
        - 
        - Get running
        - Create test script (use [repo](https://github.com/TTK4145/Project-resources))
        - json file not needed, but good for testing. As we can actually see what's happening.
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
        - Input (Elevator/Light instructions)
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
