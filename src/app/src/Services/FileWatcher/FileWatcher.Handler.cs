using System.Text.Json.Serialization;
using Microsoft.JSInterop;

namespace app.Services.FileWatcher;

public record ChangedPayload
{
    [JsonPropertyName("path")]
    public string Path { get; set; } = null!;

    [JsonPropertyName("is_dir")]
    public bool IsDir { get; set; }
}

public sealed partial class FileWatcher
{
    [JSInvokable("FileChanged")]
    public void OnFileChanged(ChangedPayload payload)
    {
        Console.WriteLine("File changed: " + string.Join(", ", payload.Path, payload.IsDir));
        Changed?.Invoke(payload);
    }
}