#include <iostream>
#include <fstream>
#include <thread>

using std::ifstream, std::cout, std::endl, std::thread;

void readFile(const char* url, long long block, int part);

int main(int argc, char** argv)
{
    cout << "read start" << endl;
    const char filename[] = R"(X:\200M.csv)";
    ifstream file(filename);
    if (file.is_open())
    {
        cout << "file is opened!" << endl;
    }

    // 하드처럼 속도가 느린 드라이브는 헤더를 왔다갔다 거리느라 오히려 속도가 느려지는듯? 한번 쭉 훑은뒤 읽으면 캐시된? 영역은 빠르게 출력하는걸 볼 수 있따. (20기가 대상)

    // char c;
    // long index = 0;
    // while(file.get(c))
    // {
    //     index++;
    // }
    // cout << index;
    file.seekg(0, std::ios::end);
    long long length = file.tellg();
    cout << length << endl;
    file.close();
    int block = length / 16;
    thread t1(readFile, filename, block, 0);
    thread t2(readFile, filename, block, 1);
    thread t3(readFile, filename, block, 2);
    thread t4(readFile, filename, block, 3);
    thread t5(readFile, filename, block, 4);
    thread t6(readFile, filename, block, 5);
    thread t7(readFile, filename, block, 6);
    thread t8(readFile, filename, block, 7);
    thread t9(readFile, filename, block, 8);
    thread t10(readFile, filename, block, 9);
    thread t11(readFile, filename, block, 10);
    thread t12(readFile, filename, block, 11);
    thread t13(readFile, filename, block, 12);
    thread t14(readFile, filename, block, 13);
    thread t15(readFile, filename, block, 14);
    thread t16(readFile, filename, block, 15);
    t1.join();
    t2.join();
    t5.join();
    t6.join();
    t7.join();
    t8.join();
    t9.join();
    t10.join();
    t11.join();
    t12.join();
    t13.join();
    t14.join();
    t15.join();
    t16.join();
    cout << "end!" << endl;
    return 0;
}

void readFile(const char* url, long long block, int part)
{
    ifstream file(url);
    char c;
    file.seekg(block * part);
    long long to = block * (part + 1);
    long long index = 0;
    for (index = 0; index < block; index++)
    {
        file >> c;
    }
    file.close();
    cout << part << " end!" << index << endl;
    return;
}