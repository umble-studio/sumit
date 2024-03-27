using System.Text.Json.Serialization;
using app.Services.FileWatcher.Payload;
using Microsoft.JSInterop;

namespace app.Services.FileWatcher;

public sealed partial class FileWatcher
{
    public delegate void ChangedHandler(ChangedEventArgs payload);
    public event ChangedHandler? Changed;

    public delegate void RenamedHandler(Payload.RenamedEventArgs payload);
    public event RenamedHandler? Renamed;

    [JSInvokable("FileChanged")]
    public void OnFileChanged(ChangedEventArgs payload)
    {
        Console.WriteLine("File changed: " + string.Join(", ", payload.Path, payload.IsDir));
        Changed?.Invoke(payload);
    }

     [JSInvokable("FileRenamed")]
    public void OnFileRenamed(Payload.RenamedEventArgs payload)
    {
        Console.WriteLine("File renamed: " + string.Join(", ", payload.Path, payload.IsDir));
        Renamed?.Invoke(payload);
    }
}