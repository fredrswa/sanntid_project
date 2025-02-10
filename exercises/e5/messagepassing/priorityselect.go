package main

import (
	"fmt"
	"sync"
	"time"
)

const tick = time.Millisecond * 100

// --- RESOURCE ROUTINE --- //
//
// This version of resourceManager holds the resource in an internal variable
// and “disables” the request channels (by setting them to nil) while the resource is checked out.
// It always prioritizes high–priority requests.
func resourceManager(takeLow chan Resource, takeHigh chan Resource, giveBack chan Resource) {
	var res Resource // our single managed resource

	// When resource is available we want both channels enabled.
	sendHigh := takeHigh
	sendLow := takeLow

	for {
		select {
		// Try high–priority first.
		case sendHigh <- res:
			// Resource handed out. Disable both channels until it is returned.
			sendHigh, sendLow = nil, nil

		// Also accept a resource being returned.
		case r := <-giveBack:
			res = r
			// Re-enable the channels.
			sendLow = takeLow
			sendHigh = takeHigh

		// If no high–priority request is waiting then try low–priority.
		default:
			select {
			case sendLow <- res:
				sendHigh, sendLow = nil, nil
			case r := <-giveBack:
				res = r
				sendHigh, sendLow = takeHigh, takeLow
			}
		}

		// When the resource is out (both channels nil) then wait for it to be returned.
		if sendHigh == nil && sendLow == nil {
			r := <-giveBack
			res = r
			sendHigh = takeHigh
			sendLow = takeLow
		}
	}
}

// --- RESOURCE TYPE --- //

type Resource struct {
	value []int // Each user appends its own id when executing.
}

// --- RESOURCE USERS --- //

type ResourceUserConfig struct {
	id        int
	priority  int
	release   int
	execution int
}

var wg sync.WaitGroup

func resourceUser(cfg ResourceUserConfig, take chan Resource, giveBack chan Resource) {
	defer wg.Done()

	// Wait until the scheduled release time.
	time.Sleep(time.Duration(cfg.release) * tick)

	executionStates[cfg.id] = waiting
	res := <-take

	executionStates[cfg.id] = executing

	// Simulate execution.
	time.Sleep(time.Duration(cfg.execution) * tick)
	res.value = append(res.value, cfg.id)
	giveBack <- res

	executionStates[cfg.id] = done
}

// --- EXECUTION LOGGING --- //

type ExecutionState rune

const (
	none      ExecutionState = ' '       // blank
	waiting   ExecutionState = '\u2592'  // ▒
	executing ExecutionState = '\u2593'  // ▓
	done      ExecutionState = '\u2580'  // ▀
)

var executionStates []ExecutionState

// executionLogger prints the state of all users every tick. It now listens for a stop signal.
func executionLogger(stop chan struct{}, loggerWg *sync.WaitGroup) {
	defer loggerWg.Done()

	// Print header.
	fmt.Printf("  id:")
	for id := range executionStates {
		fmt.Printf("%3d", id)
	}
	fmt.Println()

	t := 0
	for {
		select {
		case <-stop:
			return
		default:
			grid := ' '
			if t%5 == 0 {
				grid = '\u2500'
			}
			fmt.Printf("%04d : ", t)
			for id, state := range executionStates {
				fmt.Printf("%c%c%c", state, grid, grid)
				// Reset a completed (done) state so we see the change.
				if state == done {
					executionStates[id] = none
				}
			}
			fmt.Println()
			t++
			time.Sleep(tick)
		}
	}
}

// --- MAIN --- //

func main() {
	// Channels for resource requests and returns.
	takeLow := make(chan Resource)
	takeHigh := make(chan Resource)
	giveBack := make(chan Resource)

	// Launch the resource manager.
	go resourceManager(takeLow, takeHigh, giveBack)

	// There are 10 execution states (user IDs from 0 to 9).
	executionStates = make([]ExecutionState, 10)

	// Define resource user configurations.
	cfgs := []ResourceUserConfig{
        {0, 0, 1, 1},
        {1, 0, 3, 1},
        {2, 1, 5, 1},
        
        {0, 1, 10, 2},
        {1, 0, 11, 1},
        {2, 1, 11, 1},
        {3, 0, 11, 1},
        {4, 1, 11, 1},
        {5, 0, 11, 1},
        {6, 1, 11, 1},
        {7, 0, 11, 1},
        {8, 1, 11, 1},
        
        {0, 1, 25, 3},
        {6, 0, 26, 2},
        {7, 0, 26, 2},
        {1, 1, 26, 2},
        {2, 1, 27, 2},
        {3, 1, 28, 2},
        {4, 1, 29, 2},
        {5, 1, 30, 2},
    }

	// Launch the execution logger.
	loggerStop := make(chan struct{})
	var loggerWg sync.WaitGroup
	loggerWg.Add(1)
	go executionLogger(loggerStop, &loggerWg)

	// Launch all resource users.
	wg.Add(len(cfgs))
	for _, cfg := range cfgs {
		if cfg.priority == 1 {
			go resourceUser(cfg, takeHigh, giveBack)
		} else {
			go resourceUser(cfg, takeLow, giveBack)
		}
	}

	// Wait until all resource users are done.
	wg.Wait()

	// Give a short delay before stopping the logger.
	time.Sleep(2 * tick)
	close(loggerStop)
	loggerWg.Wait()

	// Optionally, retrieve and print the final execution order from the resource.
	// (In this design the final execution order is stored in the resource’s value.)
	select {
	case finalRes := <-takeHigh:
		fmt.Println("Final execution order:", finalRes.value)
	case finalRes := <-takeLow:
		fmt.Println("Final execution order:", finalRes.value)
	default:
		fmt.Println("Final execution order not available.")
	}
}
