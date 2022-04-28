#include "timeBenchmark.hpp"

namespace tbmark
{
    int tbmark(const char *path, bool print)
    {
        clock_t duration = timing_exec(path, print);

        return duration;
    }

    unsigned long timing_exec(const char *path, bool print)
    {
        clock_t start = clock();
        run(path, print);
        clock_t end = clock();
        return end - start;
    }

    int run(const char *path, bool print)
    {
        return std::system(path);
    }

    unsigned long Wrapper::timing_exec(int *func(void), int loopCount, clock_t duration)
    {
        clock_t start = clock();
        run(func);
        clock_t end = clock();
        return end - start;
    }
    int Wrapper::tbmark(int *func(void), int loopCount, clock_t totalTime)
    {
        clock_t duration = timing_exec(func, loopCount, totalTime);

        return duration;
    }

    int Wrapper::run(int *func(void))
    {
        return *func();
    }
}