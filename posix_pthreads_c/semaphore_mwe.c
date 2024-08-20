#include <semaphore.h>
#include <pthread.h>
#include <inttypes.h>
#include <stdio.h>
#include <unistd.h>

sem_t sem;

uint64_t counter_1 = 0;
uint64_t counter_2 = 0;

pthread_t t_1, t_2, t_3;

void* t1_impl(void* _) {
    while(counter_1 < 1000000000) {
        ++counter_1;
    }
    sem_post(&sem);

    return NULL;
}

void* t2_impl(void* _) {
    while(counter_2 < 2000000000) {
        ++counter_2;
    }
    sem_post(&sem);

    return NULL;
}

void* t3_impl(void* _) {
    sem_wait(&sem);
    sem_wait(&sem);

    printf("End: counter_1 = %" PRIu64 " counter_2 = %" PRIu64"\n",
            counter_1, counter_2);

    return NULL;
}

int main(void) {
    sem_init(&sem, 0, 0);
    pthread_create(&t_3, NULL, t3_impl, NULL);

    sleep(1);
    pthread_create(&t_1, NULL, t1_impl, NULL);
    pthread_create(&t_2, NULL, t2_impl, NULL);

    sem_destroy(&sem);
    pthread_exit(NULL);
    return 0;
}
