using app.Services.FileWatcher;
using app.Services.FileWatcher.Payload;

namespace app.Services;

public sealed class Hotreload
{
    private readonly FileWatcher.FileWatcher _fileWatcher;

    public Hotreload(FileWatcher.FileWatcher fileWatcher)
    {
        _fileWatcher = fileWatcher;
        _fileWatcher.Changed += OnFileChanged;
    }

    public ValueTask Initialize()
    {
        return _fileWatcher.Initialize();
    }

    private void OnFileChanged(ChangedPayload payload)
    {
        
    }
}