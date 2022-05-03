#include "timeBenchmark.hpp"
#include <iostream>

int main(int argc, char **argv)
{
    std::cout << tbmark::tbmark(argv[1]) << std::endl;
    // tbmark::Wrapper a = tbmark::Wrapper(myfun);
    // std::cout << a.tbmark(2, 2) << std::endl;
    return 0;
}

int myfun()
{
    std::cout << "안녕하세요" << std::endl;
    return 0;
}

int myfun2()
{
    std::cout << "안녕하세요\n";
    return 0;
}