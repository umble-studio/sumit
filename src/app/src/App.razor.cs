using app.Services.HotReload;
using Microsoft.AspNetCore.Components;

namespace app;

public sealed partial class App : ComponentBase
{
    [Inject] public HotReload HotReload { get; set; } = null!;

    protected override async Task OnAfterRenderAsync(bool firstRender)
    {
        if (!firstRender) return;
        await HotReload.Initialize();
    }
}