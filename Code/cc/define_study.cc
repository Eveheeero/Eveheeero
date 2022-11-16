#define a(w) #w // "w"가 된다.
#define myword(q) "aq" << std:## q  // std:q처럼 붙게 된다.
#define my2(a, b) a ## b // ab가 된다. 하지만 이때 a도 매크로일경우 ab에 대한 인자가 실행되기때문에 오류가 난다.
#include <iostream>

int main() {
    std::cout << myword(:endl) << std::endl;
    my2(ret,urn) 0;
}
