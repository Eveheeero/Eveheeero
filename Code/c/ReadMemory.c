#include <stdio.h>
#include <windows.h> // ��α׿��� ������ ���� �޸� �б� ���α׷�, windows api ���α׷��� �Ҷ� ������ �ɰͰ��� �������̴�.

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
    printf("0x40000 ���� %u ����Ʈ�� �о����ϴ�.\n", uRead);
  }
  CloseHandle(hHandle);
}
