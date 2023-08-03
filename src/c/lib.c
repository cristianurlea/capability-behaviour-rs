// #include <stdio.h>
#include <stdlib.h>

// char *foobar __attribute__((aligned(16))) = "asdf";
// char *foobar  = "asdf";

void donothing() {
    asm(
        " nop\n"
    );
}

int *double_input(int *input) {
    int *res = malloc(sizeof(int));

    *res=(*input) * 33;
    return res;
}