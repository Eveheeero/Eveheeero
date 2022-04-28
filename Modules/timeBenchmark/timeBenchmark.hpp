#pragma once


#include <cstdlib>
#include <time.h>

namespace tbmark {
    int tbmark(const char *path, bool print = false);
    unsigned long timing_exec(const char *path, bool print = false);
    int run(const char *path, bool print = false);
}
