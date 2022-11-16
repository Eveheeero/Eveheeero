#include <future>
#include <iostream>
#include <thread>

void
print(std::promise<int>&& promise);

int
main(int argc, char** argv)
{
  // 값을 불러오는 promise 설정
  std::promise<int> promise;
  // promise에서부터 값을 받는 함수 설정 (auto로도 설정 가능)
  std::future<int>  val = promise.get_future();

  // thread 설정
  // std::move를 사용해서 인자를 건내줘야한다.
  std::thread q(print, std::move(promise));

  // q가 종료될때까지 대기
  q.join();

  // promise에서 값 불러오기
  std::cout << val.get() << std::endl;
  return 0;
}

// &&을 사용해야 한다.
void
print(std::promise<int>&& promise)
{
  std::cout << "hello" << std::endl;

  // promise에 값 설정
  promise.set_value(1);
  return;
}