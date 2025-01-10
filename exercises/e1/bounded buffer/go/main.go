package main

import (
	"fmt"
	"sync"
	"time"
)

func producer(channel chan int, wg *sync.WaitGroup) {
	defer wg.Done()
	for i := 0; i < 10; i++ {
		time.Sleep(100 * time.Millisecond)
		fmt.Printf("[producer]: pushing %d\n", i)
		channel <- i //pushes real value to buffer
	}
	close(channel)
}

func consumer(channel chan int, wg *sync.WaitGroup) {
	defer wg.Done()
	time.Sleep(1 * time.Second)
	for {
		i := <-channel //gets real value from buffer
		fmt.Printf("[consumer]: %d\n", i)
		time.Sleep(50 * time.Millisecond)
	}
}

func main() {

	//Make a bounded buffer
	channel := make(chan int, 5)

	var wg sync.WaitGroup
	wg.Add(2)

	go consumer(channel, &wg)
	go producer(channel, &wg)

	wg.Wait()
}
