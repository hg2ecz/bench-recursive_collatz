#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

void collatz(uint64_t szam, uint64_t *lepes, uint64_t *max) {
    if (*max < szam) {
        *max = szam;
    }
    *lepes += 1; // lepes.checked_add(1).expect("Túlcsordult a számláló!");
    if (szam != 1) {
        if (szam % 2 == 0) {
            collatz(szam / 2, lepes, max);
        } else {
            collatz(3 * szam + 1, lepes, max);
        }
    }
}

int main(int argc, char **argv) {
    uint64_t maxcount = atoi(argv[1]);
    uint64_t maxnum = 0;
    for (uint64_t szam=1; szam < maxcount; szam++) {
        uint64_t lepes = 0;
        uint64_t tmpmax = 0;
        collatz(szam, &lepes, &tmpmax);
        if (maxnum < tmpmax) {
            maxnum = tmpmax;
            printf("Lépés: %lu Szám: %lu, max: %lu\n", lepes, szam, maxnum);
        }
    }
}
