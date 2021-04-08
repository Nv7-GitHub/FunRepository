#include "cbench.h"

int add(int a, int b) {
  return a + b;
}

int multiply(int a, int b) {
  int out = 0;
  for (int i = 0; i < b; i++) {
    out = add(out, a);
  }
  return out;
}