#include "stdio.h"
#include "stdlib.h"

void*
is_valid_pointer(void* ptr)
{
  FILE* f = fopen("/proc/self/maps", "r");
  if (f == NULL) {
    return NULL;
  }

  char*  line = NULL;
  size_t len  = 0;
  while (getline(&line, &len, f) != -1) {
    size_t section_from, section_to;
    sscanf(line, "%zx-%zx", &section_from, &section_to);
    if (section_from <= (size_t)ptr && (size_t)ptr <= section_to) {
      fclose(f);
      return ptr;
    }
  }
  fclose(f);

  return NULL;
}

int
main()
{
  int* p = malloc(sizeof(int));
  printf("%p\n", is_valid_pointer(p));
}