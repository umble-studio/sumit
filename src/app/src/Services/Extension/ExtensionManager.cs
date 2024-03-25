using System.Runtime.Loader;
using Microsoft.JSInterop;
using Sumit.Extension;
using Sumit.Extension.Extensions;

namespace app.Services.Extension;

public sealed class ExtensionManager(IJSRuntime js, ExtensionRegistry registry)
{
    private readonly List<Sumit.Extension.Extension> _extensions = [];
    
    public async Task LoadExtensions()
    {
        foreach (var manifest in await registry.GetManifests())
        {
            var extension = await LoadExtension(manifest);
            
            if (extension is null)
            {
                Console.WriteLine($"Failed to load extension: {manifest.Name}");
                continue;
            }

            if (IsRegistered(manifest.Name)) continue;
            _extensions.Add(extension);
            
            extension.OnLoad();
            extension.OnEnabled();
            
            Console.WriteLine($"Loaded extension: {manifest.Name}");
        }
    }

    private async Task<Sumit.Extension.Extension?> LoadExtension(ExtensionManifest manifest)
    {
        const string LocalPluginPath = "sumit-app/src/plugins/bin/Debug/net8.0/Finder.dll";

        var stream = await GetMemoryStreamFromFile(LocalPluginPath);
        if (stream is null) return null;

        var asm = AssemblyLoadContext.Default.LoadFromStream(stream);

        var mainType = asm.GetTypes().FirstOrDefault(x => x.Name is "Main");
        if (mainType is null) return null;

        if (Activator.CreateInstance(mainType, [manifest]) is not Sumit.Extension.Extension extension) return null;
        
        // Only return the extension if a component entry is provided
        return !extension.GetComponentEntry(out _) ? null : extension;
    }

    private void EnableExtension(string name)
    {
        throw new NotImplementedException();
    }

    private void DisableExtension(string name)
    {
        throw new NotImplementedException();
    }
    
    public bool IsRegistered(string name) => GetExtension(name, out _);

    public bool GetExtension(string name, out Sumit.Extension.Extension extension)
    {
        extension = _extensions.FirstOrDefault(x => x.Manifest.Name == name)!;
        if (extension is not null) return true;
        
        extension = default!;
        return false;
    }

    private async Task<MemoryStream?> GetMemoryStreamFromFile(string path)
    {
        var buffer = await js.FsReadFile(path);
        return new MemoryStream(buffer);
    }
}