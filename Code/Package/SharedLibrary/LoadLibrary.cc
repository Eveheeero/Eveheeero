#include <iostream>     // __stdcall은 함수를 끝낼 때 스택정리를 하지 않는것, __cdecl방식은 함수를 끝낼 때 스택정리를 하는것
#include <windows.h>

int main (int argc, char **argv) {
    HMODULE hmodule = LoadLibrary("makeDll.dll");    // DLL을 로드한다.
    if (hmodule == nullptr) {                       // 만약 제대로 로드되지 않았으면 종료한다.
        std::cout << "DLL 로드에 실패하였습니다." << std::endl;
        return 1;
    }
    // 불러온 라이브러리에서 "add"함수를 찾아 주소를 가져온다.
    // 함수에 대한 타입을 지정해준다. (반환값과 인자값때문에 반드시 지정해줘야한다.)
    typedef int (*MyType1)(int, int);   // 반환형이 int인, 인자를 int, int로 주는 함수의 타입이다.
    MyType1 add = (MyType1)GetProcAddress(hmodule, "add");
    int result = add(10, 20); // 불러온 함수를 다음과 같은 방법으로 사용하면 된다.

    std::cout << "result is " << result << std::endl;

    PVOID AddPointer = reinterpret_cast<void*>(GetProcAddress(hmodule, "add")); // getProcAddress는 단순히 주소를 반환하는것이기 때문에 해당 코드처럼 포인터를 저장해놔도 된다.
    // (PVOID는 void*이다.)
    int(__cdecl* add2)(int, int) = (int(__cdecl*)(int, int))AddPointer;   // __cdecl*방식으로 해당 포인터를 받아 할당하는 방법도 있다.
    result = add2(30, 40);

    std::cout << "result is " << result << std::endl;

    typedef int (__stdcall *MyType2)(int, int); // __stdcall방식으로 타입을 지정해도 된다.
    MyType2 add3 = (MyType1)GetProcAddress(hmodule, "add");
    result = add(5, 10);    // 그리고 이런 방식으로 호출한것은 __cdecl방법으로 적용되는진 모르겠다. 디버깅이 필요하다.

    std::cout << "result is " << result << std::endl;

    return 0;
}