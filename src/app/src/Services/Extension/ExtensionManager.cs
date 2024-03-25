using System.Reflection;
using System.Runtime.Loader;
using Microsoft.JSInterop;
using Sumit.Extension;
using Sumit.Extension.Extensions;

namespace app.Services.Extension;

public sealed class ExtensionManager(IJSRuntime js, ExtensionRegistry registry)
{
    public const string LocalPluginDir = @"sumit-app\src\plugins";
    // public const string LocalPluginPath = "sumit-app/src/plugins/bin/Debug/net8.0/Finder.dll";

    private readonly List<Sumit.Extension.Extension> _extensions = [];

    public async Task LoadExtensions()
    {
        await foreach (var (path, manifest) in registry.GetManifests())
        {
            var extension = await LoadExtension((path, manifest));

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

    private async Task<Sumit.Extension.Extension?> LoadExtension((string path, ExtensionManifest manifest) info)
    {
        var (relPath, manifest) = info;
        Console.WriteLine($"Loading extension: {manifest.Name} from {relPath}");

        var manifestFileRelPath = Path.Join(LocalPluginDir, relPath).Replace('\\', '/');
        var pluginDir = Path.GetDirectoryName(manifestFileRelPath)?.Split('/').Last();
        var entrypointPath = Path.Join(LocalPluginDir, pluginDir, manifest.Client.EntryPoint).Replace('/', '\\');

        var stream = await GetMemoryStreamFromFile(entrypointPath);
        if (stream is null) return null;

        var asm = AssemblyLoadContext.Default.LoadFromStream(stream);
        
        var entryType = asm.GetTypes().FirstOrDefault(x => x.IsAssignableTo(typeof(Sumit.Extension.Extension)));
        if (entryType is null) return null;
        
        if (Activator.CreateInstance(entryType) is not Sumit.Extension.Extension extension) return null;

        var ctor = entryType.GetMethod("__ctor", BindingFlags.Instance | BindingFlags.NonPublic);
        if (ctor is null) return null;

        // Inject the manifest from the internal __ctor method
        ctor.Invoke(extension, [manifest]);
        
        // Only return the extension if a component entry is provided
        return !extension.GetComponentEntry(out _) ? null : extension;
    }

    public void EnableExtension(string name)
    {
        if (!GetExtension(name, out var extension)) return;
        extension.OnEnabled();
    }

    public void DisableExtension(string name)
    {
        if (!GetExtension(name, out var extension)) return;
        extension.OnDisabled();
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