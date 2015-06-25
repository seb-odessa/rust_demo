#include <stdio.h>

extern unsigned int add(int lhs, int rhs);

int main() {
  printf("add(40,2) = %d\n", add(40,2));

  return 0;
}