using Microsoft.AspNetCore.Components;

namespace app.Components;

public partial class TextInput : ComponentBase
{
    [Parameter] public string Placeholder { get; set; }
    [Parameter] public string Value { get; set; }
}