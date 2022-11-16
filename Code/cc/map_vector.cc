#include <iostream>
#include <map>
#include <vector>
// vector 객체는 서로 다를텐데 서로 비교 가능하고 map의 키로 사용 가능한지에 대한 실험.

using namespace std;

int
main(int argc, char** argv)
{
  map<vector<int>, int> data;
  vector<int>           key1;
  vector<int>           key2;
  key1.push_back(1);
  key1.push_back(3);
  data[key1] = 45;
  cout << data[key1] << endl;
  key2.push_back(1);
  key2.push_back(3);
  cout << data[key2] << endl;
  cout << ((key1 == key2) ? "true" : "false") << endl;
  return 0;
}