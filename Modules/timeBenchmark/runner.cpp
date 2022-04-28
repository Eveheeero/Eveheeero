#include "timeBenchmark.h"
#include <iostream>

int main(int argc, char **argv)
{
    std::cout << tbmark::tbmark(argv[1]) << std::endl;
    return 0;
}