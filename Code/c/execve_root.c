#include <sys/types.h>
#include <unistd.h>
// int80을 사용하는 execve /bin/sh의 쉘코드를 분석하기 위해 만든 프로그램 (for linux)
int
main()
{
  char* shell[2];
  shell[0] = "/bin/sh";
  shell[1] = NULL;
  setuid(0); // uid gid 0은 보통 루트계정을 의미한다. 없애둬도 상관없다.
  setgid(0);
  execve(shell[0], shell, NULL); // 프로그램 주소, 인자
  return 0;
}
