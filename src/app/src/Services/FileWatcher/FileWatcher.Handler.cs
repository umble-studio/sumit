using System.Text.Json.Serialization;
using app.Services.FileWatcher.Payload;
using Microsoft.JSInterop;

namespace app.Services.FileWatcher;

public sealed partial class FileWatcher
{
    [JSInvokable("FileChanged")]
    public void OnFileChanged(ChangedPayload payload)
    {
        Console.WriteLine("File changed: " + string.Join(", ", payload.Path, payload.IsDir));
        Changed?.Invoke(payload);
    }

     [JSInvokable("FileRenamed")]
    public void OnFileRenamed(RenamedPayload payload)
    {
        Console.WriteLine("File renamed: " + string.Join(", ", payload.Path, payload.IsDir));
        Renamed?.Invoke(payload);
    }
}