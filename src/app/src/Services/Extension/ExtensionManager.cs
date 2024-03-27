using System.Linq.Expressions;
using System.Reflection;
using System.Runtime.CompilerServices;
using Microsoft.JSInterop;
using Sumit.Extension;
using Sumit.Extension.Extensions;

namespace app.Services.Extension;

public sealed class ExtensionManager(IJSRuntime js, ExtensionRegistry registry)
{
    public const string LocalPluginDir = @"sumit-app\src\plugins";

    private readonly List<(ExtensionContext ExtensionContext, Sumit.Extension.Extension Extension)> _extensions = [];

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

            Console.WriteLine($"Loaded extension: {manifest.Name}");
        }
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private async Task<Sumit.Extension.Extension?> LoadExtension((string path, ExtensionManifest manifest) info)
    {
        var (relPath, manifest) = info;

        Console.WriteLine($"Loading extension: {manifest.Name} from {relPath}");

        var manifestFileRelPath = Path.Join(LocalPluginDir, relPath).Replace('\\', '/');
        var pluginDir = Path.GetDirectoryName(manifestFileRelPath)?.Split('/').Last();
        var entrypointPath = Path.Join(LocalPluginDir, pluginDir, manifest.Client.EntryPoint).Replace('/', '\\');

        var stream = await GetMemoryStreamFromFile(entrypointPath);

        if (stream is null)
        {
            Console.WriteLine($"The file: {entrypointPath} is not a valid assembly file");
            return null;
        }

        // Cretae an assembly load context and load the assembly into it
        var alc = new ExtensionAssemblyLoadContext();
        var asm = alc.LoadFromStream(stream);
        var asmName = asm.GetName().Name;

        var entryType = asm.GetTypes().FirstOrDefault(x => x.IsAssignableTo(typeof(Sumit.Extension.Extension)));

        if (entryType is null)
        {
            Console.WriteLine($"{asmName} does not contains any class who inherits from {nameof(Sumit.Extension.Extension)}");
            return null;
        }

        if (Activator.CreateInstance(entryType) is not Sumit.Extension.Extension extension)
        {
            Console.WriteLine($"Failed to create extension instance for {asmName}");
            return null;
        }

        var ctor = entryType.GetMethod("__ctor", BindingFlags.Instance | BindingFlags.NonPublic)!;

        // Inject the manifest from the internal __ctor method
        ctor.Invoke(extension, [manifest]);

        if (IsRegistered(manifest.Name)) return null;

        var loadContext = new ExtensionContext(alc);
        _extensions.Add((loadContext, extension));

        extension.OnEnabled();
        extension.OnLoad();

        // Only return the extension if a component entry is provided
        return !extension.GetComponentEntry(out _) ? null : extension;
    }

    public void UnloadExtension(string name)
    {
        var extension = GetExtensionContext(name);

        if (extension is null)
        {
            Console.WriteLine($"Failed to unload extension {name}");
            return;
        }

        var context = extension.Value;
        context.Unload();

        Console.WriteLine($"Unload extension {name}");
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

    private (ExtensionContext ExtensionContext, Sumit.Extension.Extension Extension) TryGetExtensionInfo(string name) =>
        _extensions.FirstOrDefault(x => x.Extension.Manifest.Name == name);

    public bool GetExtension(string name, out Sumit.Extension.Extension extension)
    {
        var (_, ext) = TryGetExtensionInfo(name);
        extension = ext;

        return extension is not null;
    }

    public ExtensionContext? GetExtensionContext(string name)
    {
        var (alc, _) = TryGetExtensionInfo(name);
        return alc;
    }

    private async Task<MemoryStream?> GetMemoryStreamFromFile(string path)
    {
        var buffer = await js.FsReadFile(path);
        return new MemoryStream(buffer);
    }
}