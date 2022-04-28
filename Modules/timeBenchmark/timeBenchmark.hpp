#pragma once

#include <cstdlib>
#include <time.h>

namespace tbmark
{
    int tbmark(const char *path, bool print = false);
    unsigned long timing_exec(const char *path, bool print = false);
    int run(const char *path, bool print = false);
    class Wrapper
    {
        typedef int (Wrapper::*functype)(void);

    private:
        functype func;
        unsigned long timing_exec(int loopCount, clock_t duration);

    public:
        int tbmark(int loopCount = NULL, clock_t totalTime = NULL);
        int run();
        Wrapper(functype func_);
    };
}
