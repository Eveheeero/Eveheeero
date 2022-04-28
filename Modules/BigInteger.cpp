#include <iostream>

#define padd(a, b) reinterpret_cast<void *>(reinterpret_cast<char *>(&a) + b)
#define LENGTH 5
#define BITLENGTH (64 * LENGTH)
#define MAXLENGTH 18446744073709552000
// Long Long(__int64)에 대한 unsigned의 최대 값. (ffff ffff ffff ffff)

typedef unsigned long long Uint64_t;
struct ValueType
{
    union
    {
        Uint64_t value[LENGTH];
        bool bit[BITLENGTH];
    };
};

class BigInteger
{
private:
    ValueType storage = {};
    const Uint64_t flagMask = 0b10000000'00000000'00000000'0000000'00000000'00000000'00000000'00000000;
    const Uint64_t fullMask = 0xffff'ffff'ffff'ffff;

    bool _add_one(Uint64_t &x)
    { // 각 비트를 확인하는것으로 해서, 어떤 연산이 속도가 더 빠른지 나중에 해보기
        if (flagMask ^ x)
        {
            x = x + 1;
            return false;
        }
        x = x ^ x;
        return true;
    }

    bool _seil_from(int8_t index)   // 몇번째 자릿수부터 올림을 시작했는지 표시한다.
    {
        do
        {
            index += 1;
            if (!index)
                return 1;
            // 모든 용량이 다 차있으면
        } while (_add_one(storage.value[LENGTH - index]));
        return 0;
    } // 혹은 그만큼 비트를 어느정도 보관한다음 포인터를 1비트씩 옮긴다음 적용한다.

public:
    int get_positive(void)
    {
        // return static_cast<bool>(reinterpret_cast<char *>(storage.value)[0]);
        return storage.bit[0];
    }

    void _add(Uint64_t x)
    { // 두 수의 and연산을 통해 더하기? and연산하면 같은 비트가 있는곳은 1이 드게되는데, 그것은 0으로 채워지고 앞의 비트에 1이 추가되게된다. 그 뒤 그걸 또 and하기
        Uint64_t calc = storage.value[LENGTH - 1] & x;
        if (calc & flagMask)
        {
            if (_seil_from(1)) return;
        }
        return;
    }

    void *_ptr(void)
    {
        return reinterpret_cast<void *>(&storage);
    }
};
// xor 할때 앞 뒤 순서따라서 어셈블리어 하는것이 달라지는지 확인 (xor a, b하면 되는건데 a에 무슨 값이 저장되는것때문에 추가적인 어셈블리가 필요한지

int main(int argc, char **argv)
{
    BigInteger a;
    std::cout << a._ptr() << std::endl;
    return 0;
}