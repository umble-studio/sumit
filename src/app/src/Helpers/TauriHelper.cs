using System.Text.Json;
using Microsoft.JSInterop;

public static class TauriExtensions
{
    public async static Task<T> Invoke<T>(this IJSRuntime js, string name, Dictionary<string, object> args)
    {
        return await js.InvokeAsync<T>("invoke", name, args);
    }
    
    public async static Task<bool> FsExists(this IJSRuntime js, string path)
    {
        return await js.InvokeAsync<bool>("fs_exists", path);
    }

    public static async Task<string> FsReadTextFile(this IJSRuntime js, string path)
    {
        return await js.InvokeAsync<string>("fs_readTextFile", path);
    }

    public static async Task<byte[]> FsReadFile(this IJSRuntime js, string path)
    {
        return await js.InvokeAsync<byte[]>("fs_readFile", path);
    }

    public static async Task<string[]> FsReadDir(this IJSRuntime js, string path)
    {
        return await js.InvokeAsync<string[]>("fs_readDir", path);
    }

    public static async Task<string> GetTauriVersion(this IJSRuntime js)
    {
        return await js.InvokeAsync<string>("getTauriVersion");
    }

    public static async Task SendNotification(this IJSRuntime js, string message)
    {
        await js.InvokeVoidAsync("sendNotification", message);
    }
}