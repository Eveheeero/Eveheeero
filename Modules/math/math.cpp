#include "math.hpp"

template float math::pow<float>(float, float);

namespace math
{

    template <typename T, typename E>
    T pow(T x, E y)
    {
        return x * y;   // 미작성, for문이나 while문으로 하는것 외에도 bit로 하는것
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