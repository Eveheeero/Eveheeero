#include <iostream>
#include <map>

int*               a;
std::map<int, int> m;

int
main()
{
  a = new int[4'294'967'295]();

  // for (unsigned long i = 0; i < 4'294'967'294; i++)
  // {
  //     a[i] = i;
  //     // m[i] = i;
  //     // c[i] = 1;
  // }
  std::cin >> a[4'294'967'294];
  std::cout << a[4'294'967'294];
  return 0;
}