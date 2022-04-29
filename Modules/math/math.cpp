#include "math.hpp"

namespace math
{
    template <typename T>
    T abs(T x)
    {
        union
        {
            T value;
            bool flag;
        } type;
        type.value = x;
        type.flag = false;
        return type.value;
    }

    template <typename T, typename E>
    T pow(T x, E y)
    {
    }

    template <typename T>
    T floor(T x)
    {
    }

    template <typename T>
    T ceilf(T x)
    {
    }

    template <typename T>
    T nearbyint(T x)
    {
    }

    template <typename T>
    T round(T x)
    {
    }

    template <typename T, typename E>
    T isless(T x, E y)
    {
    }

    template <typename T>
    T signbit(T x)
    {
    }
}