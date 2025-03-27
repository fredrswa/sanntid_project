# Makefile for Elevator Peer System
# Builds release folders with four different configurations

# Directories
RELEASE_DIR := release
PEER_DIR := peer
TOOLS_DIR := tools
CONFIG_DIR := $(TOOLS_DIR)/config_files

# Release subdirectories
PEER1_DIR := $(RELEASE_DIR)/peer_1
PEER2_DIR := $(RELEASE_DIR)/peer_2
PEER3_DIR := $(RELEASE_DIR)/peer_3
PEER_LAB_DIR := $(RELEASE_DIR)/peer_lab

# Config files
CONFIG1 := config_peer_local_1.toml
CONFIG2 := config_peer_local_2.toml
CONFIG3 := config_peer_local_3.toml
CONFIG_LAB := config_peer_lab.toml

# Assigner and server directories
ASSIGNER_DIR := $(TOOLS_DIR)/assigner
SIM_SERVER_DIR := $(TOOLS_DIR)/elevatorServers

# Default target
.PHONY: all
all: clean build_all

# Build all configurations
.PHONY: build_all
build_all: build_peer1 build_peer2 build_peer3 build_peer_lab copy_tools

# Clean release directory
.PHONY: clean
clean:
	@echo "Cleaning release directory..."
	rm -rf $(RELEASE_DIR)
	mkdir -p $(RELEASE_DIR)/tools
#	mkdir -p $(RELEASE_DIR)/tools/assigner
#	mkdir -p $(RELEASE_DIR)/tools/elevatorServers

# Build peer1 with config1
.PHONY: build_peer1
build_peer1:
	@echo "Building peer_1 with $(CONFIG1)..."
	mkdir -p $(PEER1_DIR)
	cp $(CONFIG_DIR)/$(CONFIG1) $(PEER_DIR)/Config.toml
	cd $(PEER_DIR) && cargo build --release
	cp $(PEER_DIR)/target/release/peer $(PEER1_DIR)/
	cp $(PEER_DIR)/entire_system.json $(PEER1_DIR)/
	cp $(CONFIG_DIR)/$(CONFIG1) $(PEER1_DIR)/Config.toml

# Build peer2 with config2
.PHONY: build_peer2
build_peer2:
	@echo "Building peer_2 with $(CONFIG2)..."
	mkdir -p $(PEER2_DIR)
	cp $(CONFIG_DIR)/$(CONFIG2) $(PEER_DIR)/Config.toml
	cd $(PEER_DIR) && cargo build --release
	cp $(PEER_DIR)/target/release/peer $(PEER2_DIR)/
	cp $(PEER_DIR)/entire_system.json $(PEER2_DIR)/
	cp $(CONFIG_DIR)/$(CONFIG2) $(PEER2_DIR)/Config.toml

# Build peer3 with config3
.PHONY: build_peer3
build_peer3:
	@echo "Building peer_3 with $(CONFIG3)..."
	mkdir -p $(PEER3_DIR)
	cp $(CONFIG_DIR)/$(CONFIG3) $(PEER_DIR)/Config.toml
	cd $(PEER_DIR) && cargo build --release
	cp $(PEER_DIR)/target/release/peer $(PEER3_DIR)/
	cp $(PEER_DIR)/entire_system.json $(PEER3_DIR)/
	cp $(CONFIG_DIR)/$(CONFIG3) $(PEER3_DIR)/Config.toml

# Build peer_lab with config_lab
.PHONY: build_peer_lab
build_peer_lab:
	@echo "Building peer_lab with $(CONFIG_LAB)..."
	mkdir -p $(PEER_LAB_DIR)
	cp $(CONFIG_DIR)/$(CONFIG_LAB) $(PEER_DIR)/Config.toml
	cd $(PEER_DIR) && cargo build --release
	cp $(PEER_DIR)/target/release/peer $(PEER_LAB_DIR)/
	cp $(PEER_DIR)/entire_system.json $(PEER_LAB_DIR)/
	cp $(CONFIG_DIR)/$(CONFIG_LAB) $(PEER_LAB_DIR)/Config.toml

# Copy necessary tools (assigner and elevator servers)
.PHONY: copy_tools
copy_tools:
	@echo "Copying tools..."
	cp -r $(ASSIGNER_DIR)/* $(RELEASE_DIR)/tools/
	cp -r $(SIM_SERVER_DIR)/* $(RELEASE_DIR)/tools/

# Run local peers (starts all three local peers)
.PHONY: run_local
run_local:
	@echo "Starting peer_1..."
	cd $(PEER1_DIR) && ./peer true &
	@echo "Starting peer_2..."
	cd $(PEER2_DIR) && ./peer true &
	@echo "Starting peer_3..."
	cd $(PEER3_DIR) && ./peer true &
	@echo "All local peers started."

# Run lab peer
.PHONY: run_lab
run_lab:
	@echo "Starting lab peer..."
	cd $(PEER_LAB_DIR) && ./peer true &
	@echo "Lab peer started."

# Start with simulators
.PHONY: run_with_sim
run_with_sim:
	@echo "Starting simulators..."
	cd $(RELEASE_DIR)/tools/elevatorServers && ./SimElevatorServer --port 15657 --numfloors 4 &
	cd $(RELEASE_DIR)/tools/elevatorServers && ./SimElevatorServer --port 15658 --numfloors 4 &
	cd $(RELEASE_DIR)/tools/elevatorServers && ./SimElevatorServer --port 15659 --numfloors 4 &
	sleep 2
	@echo "Starting peers..."
	cd $(PEER1_DIR) && ./peer true &
	cd $(PEER2_DIR) && ./peer true &
	cd $(PEER3_DIR) && ./peer true &
	@echo "All peers and simulators started."

# Stop all running processes
.PHONY: stop
stop:
	@echo "Stopping all processes..."
	-killall peer SimElevatorServer
	@echo "All processes stopped."

# Help target
.PHONY: help
help:
	@echo "Elevator Peer System Build"
	@echo ""
	@echo "Targets:"
	@echo "  all            - Clean and build all peers (default)"
	@echo "  build_all      - Build all peer configurations"
	@echo "  build_peer1    - Build peer_1 with config1"
	@echo "  build_peer2    - Build peer_2 with config2" 
	@echo "  build_peer3    - Build peer_3 with config3"
	@echo "  build_peer_lab - Build peer_lab with config_lab"
	@echo "  clean          - Remove release directory"
	@echo "  run_local      - Start all three local peers"
	@echo "  run_lab        - Start the lab peer"
	@echo "  run_with_sim   - Start local peers with elevator simulators"
	@echo "  stop           - Stop all running processes"
	@echo "  help           - Show this help message"