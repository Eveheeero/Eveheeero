#include <cstddef>

inline size_t strlen(const char *str) noexcept
{
  size_t result;
  asm volatile (
    "xorb %%al, %%al;"
    "movq %%rbx, %%rdi;"
    "repne scasb;"
    "subq %%rbx, %%rdi;"
    "movq %%rdi, %0;"
    : "=r" (result)
    : "b" (str)
  );
  return result;
}
