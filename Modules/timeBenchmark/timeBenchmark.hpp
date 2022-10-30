#pragma once

#include <cstdlib>
#include <chrono>
#include <cmath>

namespace tbmark
{
    unsigned long long tbmark(const char *path, bool print = false);
    unsigned long long timing_exec(const char *path, bool print = false);
    int run(const char *path, bool print = false);
    class Wrapper
    {
        typedef int (Wrapper::*functype)(void);

    private:
        functype func;
        unsigned long long timing_exec(void);

    public:
        unsigned long long tbmark(int loopCount = 1, unsigned long long totalTime = 1'000'000'000);
        int run();
        Wrapper(functype func_);
    };
}
