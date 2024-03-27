namespace app.Services.FileWatcher.Payload;

public record PayloadBase
{
    public string Path { get; init; } = default!;
    public bool IsDir { get; init; }
}