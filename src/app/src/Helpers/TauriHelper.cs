using Microsoft.JSInterop;

public static class TauriExtensions
{
    public async static Task<bool> FsExists(this IJSRuntime js, string path)
    {
        return await js.InvokeAsync<bool>("fs_exists", path);
    }

    public async static Task<string> FsReadTextFile(this IJSRuntime js, string path)
    {
        return await js.InvokeAsync<string>("fs_readTextFile", path);
    }

    public async static Task<byte[]> FsReadFile(this IJSRuntime js, string path)
    {
        return await js.InvokeAsync<byte[]>("fs_readFile", path);
    }
    
    public async static Task<string> GetTauriVersion(this IJSRuntime js)
    {
        return await js.InvokeAsync<string>("getTauriVersion");
    }

    public async static Task SendNotification(this IJSRuntime js, string message)
    {
        await js.InvokeVoidAsync("sendNotification", message);
    }
}