// Compile with `gcc foo.c -Wall -std=gnu99 -lpthread`, or use the makefile
// The executable will be named `foo` if you use the makefile, or `a.out` if you use gcc directly

#include <pthread.h>
#include <stdio.h>

int i = 0;
pthread_mutex_t lock;

// Note the return type: void*
void* incrementingThreadFunction(){
    for (int j = 1; j < 1000000; j++) {
        pthread_mutex_lock(&lock);
        i++;
        pthread_mutex_unlock(&lock);
    }
}

void* decrementingThreadFunction(){
    for (int j = 1; j < 100000; j++) {
        pthread_mutex_lock(&lock);
        i--;
        pthread_mutex_unlock(&lock);
    }
}


int main(){
    // TODO: 
    // start the two functions as their own threads using `pthread_create`
    // Hint: search the web! Maybe try "pthread_create example"?
    pthread_t increment;
    pthread_t decrement;

    pthread_mutex_init(&lock, NULL);

    pthread_create(&increment, NULL, incrementingThreadFunction,NULL);
    pthread_create(&decrement, NULL, decrementingThreadFunction,NULL);

    // TODO:
    // wait for the two threads to be done before printing the final result
    // Hint: Use `pthread_join`    
    pthread_join(increment, NULL);
    pthread_join(decrement, NULL);

    
    
    printf("The magic number is: %d\n", i);

    pthread_mutex_destroy(&lock);
    return 0;
}
