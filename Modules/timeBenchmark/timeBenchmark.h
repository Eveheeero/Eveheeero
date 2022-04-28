// #pragma once

#ifndef timeBenchmark
#define timeBenchmark

#include <cstdlib>
#include <time.h>

namespace tbmark {
    int tbmark(const char *path, bool print = false);
    unsigned long timing(const char *path, bool print = false);
    int run(const char *path, bool print = false);
}

#endif