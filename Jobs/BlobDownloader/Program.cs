namespace Evh
{
  public class BlobDownloader
  {
    private HttpClient client;
    public static async Task Main()
    {
      var downloader = new BlobDownloader();
      await downloader.Download("https://www.youtube.com/watch?v=QH2-TGUlwu4");
    }

    public BlobDownloader()
    {
      client = new HttpClient();
    }

    public async Task<Boolean> Download(String url = "")
    {
      var response = await client.GetAsync(url);
      Console.WriteLine(response);
      var content = await response.Content.ReadAsStringAsync();
      Console.WriteLine(content);
      return true;
    }
  }
}