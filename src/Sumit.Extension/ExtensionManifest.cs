using System.ComponentModel.DataAnnotations;
using System.Text.Json.Serialization;

namespace Sumit.Extension;

public record ExtensionManifest
{
    [JsonPropertyName("name")] public string Name { get; init; } = null!;
    [JsonPropertyName("description")] public string Description { get; init; } = null!;
    [JsonPropertyName("version")] public string Version { get; init; } = null!;
    [JsonPropertyName("author")] public string Author { get; init; } = null!;
    [JsonPropertyName("license"), AllowedValues("MIT")] public string License { get; init; } = null!;
    [JsonPropertyName("client")] public ClientData Client { get; init; } = null!;

    public record ClientData
    {
        public string EntryPoint { get; init; }
    }
}