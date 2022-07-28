#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct bla {
  unsigned int a;
} bla;

const unsigned int *fib(int n);

void inc(struct bla *b);

struct bla *call(void);

void print(void);
