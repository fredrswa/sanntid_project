# Tools for the project
## **Executables**
### Elevator Server
This executable lets the program connect to the elevator at the real timer server.
```bash
./elevatorserver
```
Specifying ``` sim = false ```, ``` num_floors = 4 ``` and ``` port = 15657 ``` in Config.toml will either spawn the binary itself, or connect to the one already opened.
### Elevator Server Simulator
[SimElevatorServer](https://github.com/TTK4145/Simulator-v2) used in when running three local peers. Although does not need to be run by user as this is handled by the script. If the problem is struggling with this, it can be spawned in the terminal by calling.
```bash
./SimElevatorServer --port port_num --numfloors num_floors
```
Default values:
 - ``` port_num = 15657```, but is specified for running several simulator on the same machine.
 - ``` num_floors = 4 ```, but is limited to ``` 2 <= _ <= 9 ```

For testing multiple peers we need to simulate the networking properties of the lab server. We can run the script 
```bash
./simulate_labserver loopback 0.0
```
If we want to simulate order loopback, just pass ```loopback``` as an argument, and to simulate packetloss pass a float (ex. ```0.5 = 50%``` packetloss).
### Hall Request Assigner
A given binary which based on the given states of the system redistributes order to the working peers.
```bash
./hall_request_assigner -- "world_view"
```
Where ```world_view``` is a serialized string of the states. For more information on formatting and output, visit [cost_fns](https://github.com/TTK4145/Project-resources/tree/master/cost_fns).

---



