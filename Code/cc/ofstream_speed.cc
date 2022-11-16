#include <fstream>
#include <iostream>
// 파일 내부에 데이터를 저장하는 대신, 파일 이름으로 저장하면 속도가 어떤지 측정하기 위한 프로그램
using std::ofstream;

int
main(int argc, char** argv)
{
  ofstream           file;
  char               buffer[100] = R"(R:\)";
  unsigned long long hash;
  struct
  {
    unsigned long long hashs;
  } hash2;
  for (unsigned long long i = 0; i < 200000000000000000; i += 100000000) {
    snprintf(&(buffer[3]), 96, "%llu", i);
    file.open(buffer, std::ios::out);
    file.close();
  }
  return 0;
}