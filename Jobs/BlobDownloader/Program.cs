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
      var source = await this.download_url(url);
      var blob_url = this.parse_blob_url(source);
      await download_blob("result", blob_url);
      return true;
    }

    protected async Task<String> download_url(String url)
    {
      var response = await client.GetAsync(url);
      var content = await response.Content.ReadAsStringAsync();
      return content;
    }

    private String parse_blob_url(String content)
    {
      // TODO
      return "";
    }

    protected async Task download_blob(String download_to, String url)
    {
      var response = await client.GetAsync(url);
      var content = await response.Content.ReadAsByteArrayAsync();
      // TODO
    }
  }
}