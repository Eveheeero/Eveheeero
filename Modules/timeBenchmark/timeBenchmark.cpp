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
}