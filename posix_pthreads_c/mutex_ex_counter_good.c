#include <pthread.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>

// mutex mを定義
pthread_mutex_t m;

pthread_t t_1, t_2;
uint64_t value = 0;

void* impl1(void* _) {
    for(int n = 0; n < 1000000000; ++n) {
        //m をロック
        pthread_mutex_lock(&m);
        value += 1;
        // mをアンロック
        pthread_mutex_unlock(&m);
    }

    return NULL;
}

int main(void) {
    // mutexを初期化
    pthread_mutex_init(&m, NULL);

    pthread_create(&t_1, NULL, impl1, NULL);
    pthread_create(&t_2, NULL, impl1, NULL);

    pthread_join(t_1, NULL);
    pthread_join(t_2, NULL);
    printf("%"PRIu64"\n", value);

    // mutexを破棄
    pthread_mutex_destroy(&m);

    return 0;
}
