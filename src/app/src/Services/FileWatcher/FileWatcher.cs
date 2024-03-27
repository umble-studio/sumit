using app.Services.FileWatcher.Payload;
using Microsoft.JSInterop;

namespace app.Services.FileWatcher;

public sealed partial class FileWatcher
{
    private readonly DotNetObjectReference<FileWatcher> _ref = null!;
    private readonly Lazy<Task<IJSObjectReference>> moduleTask;

    public delegate void ChangedHandler(ChangedPayload payload);
    public event ChangedHandler? Changed;

    public delegate void RenamedHandler(RenamedPayload payload);
    public event RenamedHandler? Renamed;

    public FileWatcher(IJSRuntime js)
    {
        _ref = DotNetObjectReference.Create(this);

        moduleTask = new(() => js.InvokeAsync<IJSObjectReference>(
            "import", $"./js/fileWatcher.js").AsTask());
    }

    public async ValueTask Initialize()
    {
        var module = await moduleTask.Value;
        await module.InvokeVoidAsync("initialize", _ref);
    }

    public async ValueTask DisposeAsync()
    {
        if (moduleTask.IsValueCreated)
        {
            var module = await moduleTask.Value;
            await module.DisposeAsync();
        }
    }
}