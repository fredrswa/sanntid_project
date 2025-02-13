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
        - Get running
        - Create test script (use [repo](https://github.com/TTK4145/Project-resources))
    - Network:
        - Create test script
        - Figure out a package that contains info. (or use more threads).
        - What type of info do we want the network module to receive
    - FSM/IO:
        - Reconsider default statement in main loop
        - Maybe be more consistent with forcing u8, lots of recasting.
        - Modularise single elevator into modules IO and FSM in peer module.
2. main.rs:
    - A way too coordinate modules
    - Open channels between them
