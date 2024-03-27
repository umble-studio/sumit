﻿using System.Collections;
using System.Text.Json;
using Microsoft.JSInterop;
using Sumit.Extension;

namespace app.Services.Extension;

public sealed class ExtensionRegistry(IJSRuntime js)
{
    private List<ExtensionManifest> _manifests = [];

    public void Register(ExtensionManifest manifest)
    {
        if (IsRegistered(manifest.Name)) return;
        _manifests.Add(manifest);
    }

    public void Unregister(string name)
    {
        if (!IsRegistered(name)) return;
        _manifests.RemoveAll(x => x.Name == name);
    }

    public async IAsyncEnumerable<(string entry, ExtensionManifest manifest)> GetManifests()
    {
        const string dir = ExtensionManager.LocalPluginDir;

        var entries = await js.FsReadDir(dir);
        var files = entries.Where(x => x.EndsWith("manifest.json"));

        foreach (var file in files)
        {
            var manifest = await GetManifest(file);
            if (manifest is null) continue;

            yield return (file, manifest);
        }
    }

    public async Task<ExtensionManifest?> GetManifest(string path)
    {
        var content = await js.FsReadTextFile(path);
        return JsonSerializer.Deserialize<ExtensionManifest>(content)!;
    }

    private ExtensionManifest? GetExtensionManifest(string name) =>
        _manifests.FirstOrDefault(x => x.Name == name);

    public bool IsRegistered(string name) => GetExtensionManifest(name) is not null;
}