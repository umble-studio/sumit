namespace app.Services.HotReload;

public sealed partial class HotReload
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
}