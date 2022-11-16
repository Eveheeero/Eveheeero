#include <iostream> // 실행 불가능

int main(int argc, char **argv)
{
    // std::basic_ostream bo;  // class basic_ostream : virtual public basic_ios 로 선언되었는데, 그곳에 템플릿을 지켜봐야한다
    std::ostream o(nullptr); // basic_ostream형태의 인자를 받는다, 또한 형변환을 허용하지 않는다. 따라서 basic_ostream형태의 개체를 인자로 줘야하는데
                             // basic_ostream는 protected여서 구현 불가능하다.
    o << "This is osream" << std::endl;

    run();

    return 0;
}

void run()
{
    // std::basic_ostream<char, std::char_traits<char>> o; // this is protected constructure
    // o << "This is basic_osream" << std::endl;
    return;
}