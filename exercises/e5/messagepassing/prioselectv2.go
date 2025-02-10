package main

import "fmt"
import "time"


const tick = time.Millisecond * 33


// --- RESOURCE ROUTINE --- //
/*
You will modify the resourceManager routine so that it... works...
(You know the drill by now...)

Note: The code in `main` does not contain checks that the final execution order is correct.

Hints:
 - Remember that you can do things outside the `select` too:
    ```Go
    for {
        select {
            cases...
        }
        other stuff...
    }
    ```
 - You should not need a `busy` boolean (or any other variables for that matter).  
   But you might find it useful when experimenting...
 - `select` can have a `default` case. 
 - You will need to completely restructure the existing code, not just extend it
*/

type Resource struct {
    value []int // Resource type is []int. Each user appends its own id when executing.
}


func resourceManager(takeLow chan Resource, takeHigh chan Resource, giveBack chan Resource) {
	// Our managed resource.
	var res Resource // or initialize as needed

	// When the resource is available both channels are enabled.
	sendHigh := takeHigh
	sendLow := takeLow

	for {
		// First try the outer select: high–priority requests and giveBack.
		select {
		// High–priority: try to send the resource.
		case sendHigh <- res:
			// Resource has been handed out.
			// Disable both channels until the resource is returned.
			sendHigh = nil
			sendLow = nil

		// Always listen for the resource being returned.
		case r := <-giveBack:
			res = r
			// Ensure that when we have a resource available,
			// both channels are re-enabled.
			sendHigh = takeHigh
			sendLow = takeLow

		// If nothing is ready for high priority or giveBack, then try low–priority.
		default:
			select {
			// Try low–priority.
			case sendLow <- res:
				// Resource given out.
				sendHigh = nil
				sendLow = nil
			// Also check if the resource is being returned.
			case r := <-giveBack:
				res = r
				sendHigh = takeHigh
				sendLow = takeLow
			// Add a default to avoid blocking if neither low nor giveBack are available.
			default:
				// No actions available; pause briefly before re-looping.
				// This prevents a busy loop.
				time.Sleep(5 * time.Millisecond)
			}
		}

		// If the resource has been handed out (channels disabled),
		// wait (block) until it is returned.
		if sendHigh == nil && sendLow == nil {
			r := <-giveBack
			res = r
			// Re-enable both channels.
			sendHigh = takeHigh
			sendLow = takeLow
		}
	}
}
    

// --- RESOURCE USERS -- //

type ResourceUserConfig struct {
    id          int
    priority    int
    release     int
    execution   int
}

func resourceUser(cfg ResourceUserConfig, take chan Resource, giveBack chan Resource){
        
    time.Sleep(time.Duration(cfg.release) * tick)
    
    executionStates[cfg.id] = waiting
    res := <-take
    
    executionStates[cfg.id] = executing
    
    time.Sleep(time.Duration(cfg.execution) * tick)
    res.value = append(res.value, cfg.id)
    giveBack <- res
    
    executionStates[cfg.id] = done
}


func main(){
    takeLow     := make(chan Resource)
    takeHigh    := make(chan Resource)
    giveBack    := make(chan Resource)
    go resourceManager(takeLow, takeHigh, giveBack)
    
    executionStates = make([]ExecutionState, 10)

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
    
    go executionLogger()
    for _, cfg := range cfgs {
        if cfg.priority == 1 {
            go resourceUser(cfg, takeHigh, giveBack)
        } else {
            go resourceUser(cfg, takeLow, giveBack)
        }
    }
    
    // (no way to join goroutines, hacking it with sleep)
    time.Sleep(time.Duration(90) * tick)
    
    executionOrder := <-takeHigh
    fmt.Println("Execution order:", executionOrder)
}






// --- EXECUTION LOGGING --- //

type ExecutionState rune
const (
    none    ExecutionState  = '\u0020'
    waiting                 = '\u2592'
    executing               = '\u2593'
    done                    = '\u2580'
)

var executionStates []ExecutionState

func executionLogger(){
    time.Sleep(tick/2)
    t := 0
    
    
    fmt.Printf("  id:")
    for id, _ := range executionStates {
        fmt.Printf("%3d", id)
    }
    fmt.Printf("\n")
    
    for {
        grid := ' ';
        if t % 5 == 0 {
            grid = '\u2500'
        }
        fmt.Printf("%04d : ", t)
        for id, state := range executionStates {
            fmt.Printf("%c%c%c", state, grid, grid)
            if state == done {
                executionStates[id] = none
            }
        }
        fmt.Printf("\n")
        t++
        time.Sleep(tick)
    }
}