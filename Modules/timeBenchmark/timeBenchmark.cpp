#include "timeBenchmark.hpp"

namespace tbmark
{
    unsigned long long tbmark(const char *path, bool print)
    {
        unsigned long long duration = timing_exec(path, print);

        return duration;
    }

    unsigned long long timing_exec(const char *path, bool print)
    {
        std::chrono::system_clock::time_point start = std::chrono::system_clock::now();
        run(path, print);
        std::chrono::system_clock::time_point end = std::chrono::system_clock::now();
        return (end - start).count();
    }

    int run(const char *path, bool print)
    {
        return std::system(path);
    }

    unsigned long long Wrapper::timing_exec(void)
    { // 프로그램 한번 실행시간을 반환함
        std::chrono::system_clock::time_point start = std::chrono::system_clock::now();
        run();
        std::chrono::system_clock::time_point end = std::chrono::system_clock::now();
        return (end - start).count();
    }
    unsigned long long Wrapper::tbmark(int loopCount, unsigned long long totalTime)
    {
        unsigned long long duration = timing_exec();
        // 시간이 짧을경우 여러번 반복한다
        if (duration / 1000'000'000 < 1)
            return duration;
        else
        {
            int count = 9 - log10(duration);
            unsigned long long highest = duration;
            int now = 0;
            while (now < count)
            {
                unsigned long long value = timing_exec();
                if (highest < value)
                    highest = value;
            }
            return highest;
        }
    }

    int Wrapper::run()
    { // 프로그램을 실행시킴
        return (this->*func)();
    }
    Wrapper::Wrapper(functype func_)
        : func(func_)
    { // 객체 초기화
    }
}