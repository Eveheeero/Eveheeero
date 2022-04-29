#pragma once

const unsigned long long fullMask = 0xffff'ffff'ffff'ffff;

namespace math
{
    template <typename T>
    T abs(T x)
    {
        const char len = sizeof(T);
        const char bitFlagIndex = len - 1;
        union
        {
            T value;
            char flag[len];
        } type;
        type.value = x;
        if (!static_cast<signed char>(type.flag[bitFlagIndex] & 0x80))
            return type.value;
        if (false)
        {
            char index = 0;
            T mask = 0;
            while (index < len)
            {
                index += 1;
                mask <<= 8;
                mask |= 0xff;
            }
            type.value ^= mask;
            type.value += 1;
        }
        else
        {
            type.value ^= -1; // -1이 주어진 인자와 같은 타입이어야 한다. (0x80000000 ...)을 xor시킨후 1을 더해야한다.
            type.value += 1;
        }
        return type.value;
    }

    template <typename T, typename E>
    T pow(T x, E y);

    template <typename T>
    T floor(T x);

    template <typename T>
    T ceilf(T x);

    template <typename T>
    T nearbyint(T x);

    template <typename T>
    T round(T x);

    template <typename T, typename E>
    T isless(T x, E y);

    template <typename T>
    T signbit(T x);
}