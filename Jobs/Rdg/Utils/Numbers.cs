namespace Rdg.Utils;

class Numbers
{
  /// <summary>
  /// 0이 아닌 정수를 가져온다.
  /// </summary>
  public static UInt128 GetPositiveNotZero(string print)
  {
    UInt128 col;
    while (true)
    {
      try
      {
        Console.Write(print + ": ");
        string? input_ = Console.ReadLine();
        if (input_ == null)
        {
          Console.WriteLine("입력이 없습니다.");
          continue;
        }
        UInt128 input = UInt128.Parse(input_);
        if (input == 0)
        {
          Console.WriteLine("0보다 큰 숫자를 입력하세요.");

        }
        else
        {
          col = input;
          break;
        }
      }
      catch (Exception)
      {
        Console.WriteLine("알맞은 자연수를 입력하세요.");
      }
    }
    return col;
  }
}