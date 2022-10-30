#include "strlen.ipp"
#include <iostream>

int main(int argc, char** argv)
{
  std::string string;
  string = "hello world";
  std::cout << strlen(string.c_str()) << std::endl;
  return 0;
}