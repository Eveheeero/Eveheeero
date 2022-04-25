#define padd(a, b) reinterpret_cast<void *>(reinterpret_cast<char *>(&a) + b)
#define LENGTH 5
#define BITLENGTH (64 * LENGTH)
#define MAXLENGTH 18446744073709552000
// Long Long(__int64)에 대한 unsigned의 최대 값. (ffff ffff ffff ffff)

struct ValueType
{
    unsigned __int64 value[LENGTH];
};

class BigInteger
{
private:
    ValueType storage;
public:
    int get_positive(void)
    {
        return ((char *)storage.value)[0];
    }

    void _add(unsigned __int64 x)
    {   // 두 수의 and연산을 통해 더하기? and연산하면 같은 비트가 있는곳은 1이 드게되는데, 그것은 0으로 채워지고 앞의 비트에 1이 추가되게된다. 그 뒤 그걸 또 and하기

        return;
    }
};