#include <stdio.h>
#include <stdlib.h>

int
main()
{
  fputs("0x1234567890ab is my Pointer.\r", stdout);
  char* restrict buf = malloc(20);
  sprintf(buf, "%p\n", stdout);
  fputs(buf, stdout);
  return 0;
}