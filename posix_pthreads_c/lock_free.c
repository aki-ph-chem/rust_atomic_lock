#include <stdio.h>
#include <stdbool.h>

// 比較して交換
bool compare_and_swap(int* p, int old, int new) {
    if(*p != old) {
        return false;
    }

    *p = new;
    return true;
}

int compare_and_add(int* p, int add) {
    bool done = false;
    int value = 0;
    while(!done) {
        value = *p;
        done = compare_and_swap(p, value, value + add);
    }

    return value + add;
}

int main(void) {
    int num = 3;
    // 交換されない
    compare_and_swap(&num, 2, 12);
    printf("num = %d\n", num);
    // 交換される
    compare_and_swap(&num, 3, 12);
    printf("num = %d\n", num);

    compare_and_add(&num, 100);
    printf("num = %d\n", num);
}
