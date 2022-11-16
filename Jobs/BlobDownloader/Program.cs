namespace Evh
{
  public class BlobDownloader
  {
    private HttpClient client;
    public static void Main()
    {
      Console.WriteLine("Start!");
      var downloader = new BlobDownloader();
      downloader.Download("https://www.youtube.com/watch?v=QH2-TGUlwu4");
    }

    public BlobDownloader()
    {
      client = new HttpClient();
    }

    public Boolean Download(String url = "")
    {
      var response = client.GetAsync(url).Result;
      Console.WriteLine(response);
      return true;
    }
  }
}