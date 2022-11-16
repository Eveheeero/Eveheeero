#include <iostream>
// g++ ASM.cpp -S -o ASM.S // CPP를 어셈블리로 컴파일
// g++ ASM.cpp -c -o ASM.o // 컴파일 하고 링킹은 안하기
// as a1.s // 어셈블리어 컴파일하기
// g++ a1.out ASM.o // 컴파일한것들 링킹하기

extern int a1();    // 두글자면 _Z2a1v, 네글자면 _Z4testv, C스타일이면 이름만
extern "C" int a2();    // C스타일이라 이름은 그대로다.
extern char a3(int*, char*);
// extern bool notUse(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int);
// 인자테스트이다. notUse(edi, esi, edx, ecx, r8d, r9d, push push, push... 순으로 들어간다), 역순으로 들어가게 된다.
// 앞에는 _Z가 온 뒤, 함수 글자수, 인자종류가 오게 된다. (int가 4개면 iiii, void면 v)
// long은 l, double은 d, unsigned char은 h. 이때 double은 pxor%xmm0, %xmm0명령이 오게 되었다.
// 포인터는 Pi, Pc등으로 P가 오게된다.
int main()
{
    std::cout << a1() << std::endl;
    std::cout << a2() << std::endl;
    std::cout << a3(0, 0) << std::endl;
    return 0;
}