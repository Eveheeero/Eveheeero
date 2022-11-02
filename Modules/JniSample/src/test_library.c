#include "App.h" // Write your code with header
// If could not find jni.h, use -I option to /usr/lib/jvm/<Java version>/include
// If cound not find jni_md.h, use -I option to /usr/lib/jvm/<Java version>/include/linux

// When you Don't know how to write JNI code, see header file, and match arguments.
JNIEXPORT int JNICALL                               // Publish function need JNIEXPORT keyword
Java_App_mul(JNIEnv* env, jclass cls, int a, int b) // Function name should be Java_<Class>_<Function>
{
  return a * b;
}

#ifndef _WIN32
#include <setjmp.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>

jmp_buf unexpected_jmp;

/**
 * @brief 비정상적인 오류 핸들러, 오류가 발생했을시 롱점프 실행
 */
void
call_unexpected_error(int signal, siginfo_t* si, void* arg)
{
  longjmp(unexpected_jmp, 1);
}

#endif

// 비정상적인 오류 핸들러 적용 함수
void
init_unexpected_error()
{
#ifndef _WIN32
  // 리눅스버전 오류 핸들러 등록
  // 윈도우버전 오류 핸들러는 Winapi의 SEH를 사용한다. 필요시 작성 필요
  struct sigaction sa;
  memset(&sa, 0, sizeof(struct sigaction));
  sigemptyset(&sa.sa_mask);
  sa.sa_sigaction = call_unexpected_error;
  sa.sa_flags     = SA_SIGINFO;

  sigaction(SIGSEGV, &sa, NULL); /* 잘못된 저장공간 접근 에러 (메모리 에러) */
  sigaction(SIGFPE, &sa, NULL);  /* 잘못된 연산 */
#endif
}

// 비정상적인 오류 핸들러 삭제 함수
void
reset_unexpected_error()
{
#ifndef _WIN32
  // 비정상적인 오류 핸들러 샂게
  signal(SIGSEGV, SIG_DFL);
  signal(SIGFPE, SIG_DFL);
#endif
}

// 비정상적인 오류 핸들러 적용 매크로
#define IF_INIT_UNEXPECTED_ERROR(x)                                                                                                        \
  init_unexpected_error();                                                                                                                 \
  if (setjmp(unexpected_jmp) == 1) {                                                                                                       \
    x;                                                                                                                                     \
  }

JNIEXPORT void JNICALL
Java_App_init(JNIEnv* env, jclass cls)
{
  // 비정상적인 오류 핸들러 적용
  IF_INIT_UNEXPECTED_ERROR({
    reset_unexpected_error();
    return;
  });

  // 비정상적인 오류 핸들러 해제
  reset_unexpected_error();

  return;
}