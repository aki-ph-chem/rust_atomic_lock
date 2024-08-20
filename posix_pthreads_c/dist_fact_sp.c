#include <pthread.h>
#include <stdint.h>
#include <unistd.h>
#include <stdio.h>
#include <inttypes.h>
#include <malloc.h>

// numの約数の個数を数える
uint64_t factors(uint64_t num) {
    uint64_t result = 0;
    for(uint64_t i = 1; i <= num; ++i) {
        if(num % i == 0) {
            ++result;
        }
    }

    return result;
}

// シングルスレッドで約数の個数を数え上げる
int main(void) {
    // 定数の伝搬を防ぐためにvaolatile
    volatile uint64_t input = 2000000000;

    printf("Factors of %"PRIu64": %"PRIu64"\n", input, factors(input));

    return 0;
}
