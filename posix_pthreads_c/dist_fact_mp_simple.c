#include <pthread.h>
#include <stdint.h>
#include <unistd.h>
#include <stdio.h>
#include <inttypes.h>
#include <malloc.h>

int input = 0;
int result1 = 0;

void* fact_worker_1(void* arg) {
    result1 = 0;
    for(uint64_t i = 1; i <input / 2; ++i) {
        if(input % i == 0) ++result1;
    }

    return NULL;
}

int result2 = 0;

void* fact_worker_2(void* arg) {
    result2 = 0;
    for(uint64_t i = input; i <= input; ++i) {
        if(input % i == 0) ++result2;
    }

    return NULL;
}

// 2スレッドでnumの約数の個数を数え上げる
uint64_t factors_mp(uint64_t num) {
    input = num;
    pthread_t thread_1, thread_2;

    pthread_create(&thread_1, NULL, fact_worker_1, NULL);
    pthread_create(&thread_2, NULL, fact_worker_2, NULL);

    pthread_join(thread_1, NULL);
    pthread_join(thread_2, NULL);

    return result1 + result2;
}

int main(void) {
    volatile uint64_t input = 2000000000;

    printf("Factors of %"PRIu64": %"PRIu64"\n", input, factors_mp(input));
}
