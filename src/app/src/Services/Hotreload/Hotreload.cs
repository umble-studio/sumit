using app.Services.FileWatcher;
using app.Services.FileWatcher.Payload;

namespace app.Services.HotReload;

public sealed class HotReload
{
    private readonly FileWatcher.FileWatcher _fileWatcher;

    public HotReload(FileWatcher.FileWatcher fileWatcher)
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