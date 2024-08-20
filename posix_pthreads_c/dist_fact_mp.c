#include <pthread.h>
#include <stdint.h>
#include <time.h>
#include <unistd.h>
#include <stdio.h>
#include <inttypes.h>
#include <malloc.h>

#define THREADS 4

struct FactTask {
    uint64_t num;
    uint64_t from, to;
    uint64_t result;
};

void* fact_worker(void* arg) {
    struct FactTask* const task = arg;
    task->result = 0;
    for(uint64_t i = task->from; i < task->to; ++i) {
        if (task->num %i == 0) {
            ++(task->result);
        }
    }

    return NULL;
}

// thread_count < numを前提とする
uint64_t factors_mp(uint64_t num, size_t thread_count) {
    struct FactTask* tasks = malloc(thread_count * sizeof(*tasks));
    pthread_t* threads = malloc(thread_count * sizeof(*threads));

    uint64_t start = 1;
    // tasksを初期化する
    size_t step = num / thread_count;
    for(size_t i = 0; i < thread_count; ++i) {
        tasks[i].num = num;
        tasks[i].from = start;
        tasks[i].to = start + step;
        start += step;
    }

    // スレッドを建てる
    for(size_t i = 0; i < thread_count; ++i) {
        pthread_create(threads + i, NULL, fact_worker, tasks + i);
    }

    // スレべてのスレッドを待機
    uint64_t result = 0;
    for(size_t i = 0; i < thread_count; ++i) {
        pthread_join(threads[i], NULL);
        result += tasks[i].result;
    }

    free(tasks);
    free(threads);

    return result;
}

int main(void) {
    volatile uint64_t input = 2000000000;
    printf("Factors of %"PRIu64": %"PRIu64"\n", input, factors_mp(input, THREADS));

    return 0;
}
