#include "byte_len.ii"
#include "math.ii"
#include <iostream>

using namespace evh;

int
main(void)
{
  std::cout << byte_len_asm("Hello, world!") << std::endl;
  return 0;
}