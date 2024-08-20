#include <pthread.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>
#include <stdbool.h>

pthread_cond_t condvar = PTHREAD_COND_INITIALIZER;
pthread_mutex_t m;

bool sent = false;
void* t1_impl(void* _) {
    pthread_mutex_lock(&m);
    puts("Thread2 before wait");

    while(!sent) {
        pthread_mutex_unlock(&m);
    }

    puts("Thread2 after wait");
    pthread_mutex_unlock(&m);
    return NULL;
}

void* t2_impl(void* _){
    pthread_mutex_lock(&m);
    puts("Thread1 before signal");

    sent = true;
    pthread_cond_signal(&condvar);

    puts("Thread1 after signal");
    pthread_mutex_unlock(&m);
    return NULL;
}

int main(void) {
    pthread_t t_1, t_2;

    pthread_mutex_init(&m, NULL);
    pthread_create(&t_1, NULL, t1_impl, NULL);
    sleep(2);

    pthread_create(&t_2, NULL, t2_impl, NULL);

    pthread_join(t_1, NULL);
    pthread_join(t_2, NULL);

    pthread_mutex_destroy(&m);
    return 0;
}
