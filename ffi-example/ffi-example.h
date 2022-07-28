#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct bla {
  unsigned int a;
};

extern "C" {

const unsigned int *fib(int n);

void inc(bla *b);

bla *call();

void print();

} // extern "C"
