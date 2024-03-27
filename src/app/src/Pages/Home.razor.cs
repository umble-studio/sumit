using app.Services.Extension;
using Microsoft.AspNetCore.Components;
using Microsoft.JSInterop;
using Sumit.Extension.Extensions;

namespace app.Pages;

public partial class Home : ComponentBase
{
    private Type? _componentType;

    [Inject] public IJSRuntime Js { get; set; } = null!;
    [Inject] public HttpClient HttpClient { get; set; } = null!;
    [Inject] public ExtensionManager Extensions { get; set; } = null!;

    protected override async Task OnAfterRenderAsync(bool firstRender)
    {
        var paths = await Js.Invoke<List<byte>>("paths", new Dictionary<string, object>());
        Console.WriteLine("Greet: " + paths.Count);

        await Extensions.LoadExtensions();

        if (!Extensions.GetExtension("Finder", out var extension)) return;
        if (!extension.GetComponentEntry(out var component)) return;

        if (_componentType != component.Type)
        {
            _componentType = component.Type;
            StateHasChanged();
        }
    }

    private void UnloadExtension()
    {
        Console.WriteLine("Unload extension");

        _componentType = null;
        Extensions.UnloadExtension("Finder");
    }
}