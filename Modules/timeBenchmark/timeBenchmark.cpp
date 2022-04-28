#include "timeBenchmark.h"

namespace tbmark
{
    int tbmark(const char *path, bool print)
    {
        clock_t duration = timing(path, print);

        return duration;
    }

    unsigned long timing(const char *path, bool print)
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