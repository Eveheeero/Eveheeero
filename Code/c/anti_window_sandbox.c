#include <shlobj_core.h>
#include <stdio.h>
#include <windows.h>

#pragma comment(lib, "shell32.lib")

enum IsSandbox
{
  IS_SANDBOX     = 1,
  IS_NOT_SANDBOX = 0,
  IS_UNKNOWN     = -1
};

enum IsSandbox
detect_sandbox_default_username()
{
  /* 버퍼 생성 */
  LPWSTR path;
  path = (LPWSTR)malloc(MAX_PATH * sizeof(WCHAR));
  memset(path, 0, MAX_PATH * sizeof(WCHAR));

  /* 사용자 이름 가져오기 */
  if (SHGetSpecialFolderPathW(NULL, path, CSIDL_LOCAL_APPDATA, FALSE) == FALSE) {
    return IS_UNKNOWN;
  }

  /* 사용자 이름이 MSDN 보안 정책 전용 계정인 WDAGUtilityAccount인지 확인 */
  if (wcsstr(path, L"WDAGUtilityAccount") != NULL) {
    // 검색이 되면
    return IS_SANDBOX;
  } else {
    // 감색이 되지 않으면
    return IS_NOT_SANDBOX;
  }
}

enum IsSandbox
detect_from_nt_query_system_information()
{
}

enum IsSandbox
detect_from_cpuid()
{
  // CpuIdEx
}

int
main(int argc, char** argv)
{
  switch (detect_sandbox_default_username()) {
    case IS_SANDBOX:
      printf("Sandbox\n");
      break;
    case IS_NOT_SANDBOX:
      printf("Not Sandbox\n");
      break;
    case IS_UNKNOWN:
      printf("Unknown\n");
      break;
  }
}