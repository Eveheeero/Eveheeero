#include "byte_len.ii"
#include "decimal.hh"
#include "math.ii"
#include <iostream>

using namespace evh;

int
main(void)
{
  std::cout << byte_len_asm("Hello, world!") << std::endl;
  auto w = Decimal("12345678901234567890123456789012345678901234567890123456789012345678901234567890");
  std::cout << w.to_string() << std::endl;
  return 0;
}