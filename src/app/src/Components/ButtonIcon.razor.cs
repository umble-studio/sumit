using Microsoft.AspNetCore.Components;

namespace app.Components;

public partial class ButtonIcon : ComponentBase
{
    [Parameter] public Dictionary<string, object> Attributes { get; set; } = null!;
    [Parameter] public string Icon { get; set; } = null!;
}