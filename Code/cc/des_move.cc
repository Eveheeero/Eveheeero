#include <iostream>
using namespace std;

int
main()
{
  // 간단하게 des move연산을 하기 위한 프로그램, su[0]은 프로그램 입력을 간단하게 하기 위해 넣은
  // 임의의 수이다.
  int su[57] = { 999, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                 1,   1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0,
                 0,   1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0 };
  cout << su[14] << su[17] << su[11] << su[24] << su[1] << su[5] << su[3] << su[28] << su[15]
       << su[6] << su[21] << su[10] << su[23] << su[19] << su[12] << su[4] << su[26] << su[8]
       << su[16] << su[7] << su[27] << su[20] << su[13] << su[2] << su[41] << su[52] << su[31]
       << su[37] << su[47] << su[55] << su[30] << su[40] << su[51] << su[45] << su[33] << su[48]
       << su[44] << su[49] << su[39] << su[56] << su[34] << su[53] << su[46] << su[42] << su[50]
       << su[36] << su[29] << su[32] << endl;
  return 0;
}
