// https://learn.microsoft.com/ko-kr/dotnet/csharp/language-reference/xmldoc/recommended-tags

using System.Data;

namespace Rdg
{
  static internal class Program
  {
    static void Main()
    {
      UInt128 col, row;
      Console.WriteLine("Random Data Generator");
      Console.WriteLine("생성할 데이터의 열 수를 입력하세요.");
      col = Utils.Numbers.GetPositiveNotZero("열 수");
      row = Utils.Numbers.GetPositiveNotZero("행 수");
      var df = new DataTable();
      for (UInt128 i = 0; i < col; i++)
      {
        df.Columns.Add(i.ToString());
      }
      for (UInt128 i = 0; i < row; i++)
      {
        df.Rows.Add(df.NewRow());
      }

      foreach (DataRow row_ in df.Rows)
      {
        Console.WriteLine(row_[0]);
      }

      var q = from line in df.AsEnumerable()
              where line["Column1"].ToString() == "1"
              select line;
    }
  }
}
