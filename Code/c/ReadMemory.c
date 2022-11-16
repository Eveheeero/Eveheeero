#include <stdio.h>
#include <windows.h> // 블로그에서 가져온 원격 메모리 읽기 프로그램, windows api 프로그래밍 할때 도움이 될것같아 저장중이다.

int
main()
{
  HANDLE hHandle;
  char   szBuff[4096];
  size_t uRead;
  int    uPid = 24800;

  hHandle = OpenProcess(PROCESS_VM_READ, FALSE, uPid);
  if (hHandle == INVALID_HANDLE_VALUE)
    exit(-1);

  if (ReadProcessMemory(hHandle, (void*)0x40000, szBuff, sizeof(szBuff), &uRead)) {
    printf("0x40000 에서 %u 바이트를 읽었습니다.\n", uRead);
  }
  CloseHandle(hHandle);
}
