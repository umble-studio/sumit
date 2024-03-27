using app.Services;
using Microsoft.AspNetCore.Components;

namespace app;

public sealed partial class App : ComponentBase
{
    [Inject] public Hotreload Hotreload { get; set; } = null!;

    protected override async Task OnAfterRenderAsync(bool firstRender)
    {
        if (!firstRender) return;
        await Hotreload.Initialize();
    }
}