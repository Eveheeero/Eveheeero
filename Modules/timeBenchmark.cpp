#include "timeBenchmark.h"

namespace tbmark
{
    int tbmark(char *path)
    {
        std::system(path);
        return 0;
    }
}