#include <iostream>
#include <thread>

int
print();

int
main(int argc, char** argv)
{
  // thread 생성 및 실행
  std::thread q(print);
  std::thread w(print);

  // thread 수거
  q.join();
  w.join();
  return 0;
}

int
print()
{
  std::cout << "hello" << std::endl;
  return 0;
}