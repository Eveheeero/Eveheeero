#include <iostream>
using namespace std;
int main(int argc, char **argv)
{
    struct ab
    {
        long a[3];
    };
    ab mystruct = {1,3,5};
    cout << reinterpret_cast<void *>(reinterpret_cast<char *>(&mystruct) + 1) << endl; // void형태의 타입이면 +1을 할때 얼마나 더해야하는지 알 수 없고, char*형태의 타입이면 +1할때 1만큼 더해지지만 cout에 들어갈때
    // char의 주소가 되므로 문자열로 출력해서 나오게 된다. 즉 char*로 더해준 다음 void*로 출력하거나 long*으로 출력하는등 해야한다. (char*만 아니면 된다)
    // []을 통해서 n번째 주소의 값에 접근할 수 있을듯하다.
    // mystruct가 있는 () 안에 +1을 해주면 ab의 크기만큼 더해진다.
    cout << &mystruct.a[1] << endl;
    cout << &mystruct.a[2] << endl;
    return 0;
}