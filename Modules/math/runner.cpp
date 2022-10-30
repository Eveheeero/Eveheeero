#include "math.hpp"
#include <iostream>

int main(int argc, char **argv)
{
    std::cout << math::abs<int>(-3) << std::endl;
    return 0;
}