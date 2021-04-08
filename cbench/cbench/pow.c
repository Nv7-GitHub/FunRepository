#include "cbench.h"

int intPow(int a, int b) {
  int out = 1;
  for (int i = 0; i < b; i++) {
    out = multiply(out, a);
  }
  return out;
}