#include <iostream>
int
main(int argc, char** argv)
{
  std::cerr << "cerr테스트입니다." << std::endl;
  return 0;
}
// cerr은 cerr과 연결된 버퍼를 오류출력으로 출력하며, cerr.tie()는 cout과 똑같다. (cout은
// cerr.tie다.) cerr.flag & unitbuf는 0이 아니다.