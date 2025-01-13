// Use `go run foo.go` to run your program

package main

import (
	. "fmt"
	"runtime"
	"sync"
)

var i = 0
var a_mutex sync.Mutex

func incrementing(i_channel chan int, wg *sync.WaitGroup) {
	defer wg.Done()
	for j := 0; j < 1000000; j++ {
		i_channel <- 1
	}
}

func decrementing(i_channel chan int, wg *sync.WaitGroup) {
	defer wg.Done()
	for j := 0; j < 100000; j++ {
		i_channel <- -1
	}
}

func main() {
	// What does GOMAXPROCS do? What happens if you set it to 1?
	runtime.GOMAXPROCS(2)

	//Creates a channel where you can send and receive values (make(chan int, x) creates a channel with a buffer size of x)
	i_channel := make(chan int)

	//Creates a waitgroup which is used to wait for both functions to finish
	var wg sync.WaitGroup
	wg.Add(2)

	//Spawn both functions as goroutines
	go incrementing(i_channel, &wg)
	go decrementing(i_channel, &wg)

	//Wait for both functions to finish
	go func() {
		wg.Wait()
		close(i_channel)
	}()

	//Sum up the values. Continuesly receive values from the channel
	for val := range i_channel {
		i += val
	}

	Println("The magic number is:", i)
}
