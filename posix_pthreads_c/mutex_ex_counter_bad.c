#include <pthread.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>

// mutexの例

pthread_t t_1, t_2;
uint64_t value = 0;

void* impl1(void* _) {
    for(int n = 0; n < 1000000000; ++n) {
        value += 1;
    }

    return NULL;
}

// データ競合が起こって実行するたびに違う値が出る
int main(void) {
    pthread_create(&t_1, NULL, impl1, NULL);
    pthread_create(&t_2, NULL, impl1, NULL);

    pthread_join(t_1, NULL);
    pthread_join(t_2, NULL);
    printf("%"PRIu64"\n", value);

    return 0;
}
