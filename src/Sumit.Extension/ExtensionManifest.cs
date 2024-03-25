using System.Text.Json.Serialization;

namespace Sumit.Extension;

public record ExtensionManifest
{
    [JsonPropertyName("name")] public string Name { get; init; }
    [JsonPropertyName("description")] public string Description { get; init; }
    [JsonPropertyName("version")] public string Version { get; init; }
    [JsonPropertyName("author")] public string Author { get; init; }
    [JsonPropertyName("license")] public string License { get; init; }
}