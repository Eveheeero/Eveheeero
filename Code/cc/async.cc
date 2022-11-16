#include <future>
#include <iostream>
#include <thread>

int
print();

int
main(int argc, char** argv)
{
  // async를 설정하면 리턴값까지 받아올 수 있다.
  std::future<int> val = std::async(print);

  // get을 사용할때 받아온다.
  std::cout << val.get() << std::endl;
  return 0;
}

int
print()
{
  std::cout << "hello" << std::endl;
  return 1;
}