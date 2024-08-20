#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

void* threadimpl(void* arg) {
    for(int i = 0; i < 10; ++i) {
        puts(arg);
        sleep(1);
    }
    return NULL;
}

// スレッドを建てる
int main(void) {
    pthread_t t_1, t_2;
    pthread_create(&t_1, NULL, threadimpl, "fizz");
    pthread_create(&t_2, NULL, threadimpl, "buzz");
    pthread_exit(NULL);
    puts("bye");

    return 0;
}
