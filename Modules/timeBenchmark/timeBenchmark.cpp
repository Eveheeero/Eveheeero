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

    unsigned long Wrapper::timing_exec(int loopCount, clock_t duration)
    {   // 프로그램 한번 실행시간을 반환함
        clock_t start = clock();
        run();
        clock_t end = clock();
        return end - start;
    }
    int Wrapper::tbmark(int loopCount, clock_t totalTime)
    {
        clock_t duration = timing_exec(loopCount, totalTime);
        // 시간이 짧을경우 여러번 반복한다
        return duration;
    }

    int Wrapper::run()
    {   // 프로그램을 실행시킴
        return (this->*func)();
    }
    Wrapper::Wrapper(functype func_)
        : func(func_)
    {   // 객체 초기화
    }
}