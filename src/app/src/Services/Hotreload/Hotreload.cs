using app.Services.Extension;

namespace app.Services.HotReload;

public sealed partial class HotReload
{
    private readonly FileWatcher.FileWatcher _fileWatcher;
    private readonly ExtensionManager _extensionManager;

    public HotReload(FileWatcher.FileWatcher fileWatcher, ExtensionManager extensionManager)
    {
        _fileWatcher = fileWatcher;
        _extensionManager = extensionManager;
        
        _fileWatcher.Changed += OnFileChanged;
    }

    public ValueTask Initialize()
    {
        return _fileWatcher.Initialize();
    }
}