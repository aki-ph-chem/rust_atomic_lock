#include <pthread.h>
#include <semaphore.h>
#include <stdio.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdlib.h>
#include <time.h>

sem_t sem_begin_0, sem_begin_1, sem_end;
int x, y, read_0, read_1;

// asmはGNU拡張なためコンパイルを通すには`-std=gnu99 or -std=gnu11`が必要

void* thread_0_impl(void* param) {
    for(;;) {
        sem_wait(&sem_begin_0);
        x = 1;
        // コンパイラによる並び替えだけを防ぐ
        //asm volatile("" ::: "memory");
        __asm__ volatile("" ::: "memory");
        read_0 = y;

        sem_post(&sem_end);
    }

    return NULL;
}

void* thread_1_impl(void* param) {
    for(;;) {
        sem_wait(&sem_begin_1);
        y = 1;
        // コンパイラによる並び替えだけを防ぐ
        //asm volatile("" ::: "memory");
        __asm__ volatile("" ::: "memory");

        // これはプロセッサによる並び替えも防ぐ
        //asm volatile("mfence" ::: "memory");
        read_1 = x;

        sem_post(&sem_end);
    }

    return NULL;
}

int main(void) {
    sem_init(&sem_begin_0, 0, 0);
    sem_init(&sem_begin_1, 0, 0);
    sem_init(&sem_end, 0, 0);

    pthread_t thread_0, thread_1;
    pthread_create(&thread_0, NULL, thread_0_impl, NULL);
    pthread_create(&thread_1, NULL, thread_1_impl, NULL);

    for(uint64_t i = 0; i < 1000000; ++i) {
        x = 0;
        y = 0;
        sem_post(&sem_begin_0);
        sem_post(&sem_begin_1);

        sem_wait(&sem_end);
        sem_wait(&sem_end);

        if(read_0 == 0 && read_1 == 0) {
            printf("reordering happend on iteration %" PRIu64 "\n",i);
            exit(0);
        }
    }

    puts("No reordering detected during 1000000 iteration");
    return 0;
}
