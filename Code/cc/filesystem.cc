#include <filesystem>
#include <iostream>

int
main(int argc, char** argv)
{
  auto w = std::filesystem::current_path();
  std::cout << w.string();
}