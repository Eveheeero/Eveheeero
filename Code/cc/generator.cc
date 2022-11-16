#include <iostream>
#include <coroutine>
#include <vector>

std::generator<int> myGen()   // after g++ 11.1, -std=c++20, -fcoroutines no name of type
{
    for (int i = 0; i < 100; ++i)
        co_yield i;
}

int main(int argc, char **argv)
{
    const auto gen = myGen();
    std::cout << "Generating..." << std::endl;
    std::cout << sizeof(gen) << std::endl;

    return 0;
}