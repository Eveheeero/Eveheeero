#include <chrono>
#include <iostream>

int
main(int argc, char** argv)
{
  auto t     = std::chrono::high_resolution_clock::now();
  auto p     = std::chrono::time_point_cast<std::chrono::time_point<std::chrono::system_clock>>(t);
  auto start = std::chrono::system_clock::to_time_t(p);
  std::cout << start << std::endl;
  return 0;
}