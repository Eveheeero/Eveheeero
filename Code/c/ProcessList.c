#include <Tlhelp32.h>
#include <stdio.h> // 현재 실행중인 프로세스 리스트를 구하는 프로그램.
#include <windows.h>

int
main()
{
  HANDLE         hProcess = NULL;
  PROCESSENTRY32 pe32     = { 0 };
  hProcess                = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
  pe32.dwSize             = sizeof(PROCESSENTRY32);

  if (Process32First(hProcess, &pe32)) {
    do {
      printf("%d  %s\n", pe32.th32ProcessID, pe32.szExeFile);
    } while (Process32Next(hProcess, &pe32));
  }
  CloseHandle(hProcess);
}
