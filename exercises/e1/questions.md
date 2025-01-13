Exercise 1 - Theory questions
-----------------------------

### Concepts

What is the difference between *concurrency* and *parallelism*?
> Concurreny means running different tasks at the "same time". Quick switching between them is regarded as "same time". Parallelism mean performing tasks besides one another and not intertwined. Tasks can be concurrent and not parallel and vise versa. E.g. running tasks on threads, quickly switching between them is concurrent but not parallel. Running the same task in parallel is parallel but not concurrent.

What is the difference between a *race condition* and a *data race*? 
> A race condition is the "race between processes" to execute in the right order. A data race is when two processes access the same location in memory at the same time and minimum one of them is a write.
 
*Very* roughly - what does a *scheduler* do, and how does it do it?
> A scheduler assigns when tasks are to be executed, ensuring quickest execution of tasks while performing things in the necessary order.


### Engineering

Why would we use multiple threads? What kinds of problems do threads solve?
> Threads solve the problem of solving two tasks simultaneously. We use threads to have tasks run simultaneously without explicitly intertangle the tasks in code, keeping them separated. 

Some languages support "fibers" (sometimes called "green threads") or "coroutines"? What are they, and why would we rather use them over threads?
> Fibers run on the same thread. Fibers need to yield execution in order for another fiber to operate, sort of like using mutex on threads. Threads usually run preemtively which means that a scheduler can take access if the computational power at any desired moment, stopping the execution of a thread even if it is "unwanted". 

Does creating concurrent programs make the programmer's life easier? Harder? Maybe both?
> Yes.

What do you think is best - *shared variables* or *message passing*?
> Message passing seems safer. 


