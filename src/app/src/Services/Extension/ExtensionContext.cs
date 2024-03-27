using System.Runtime.CompilerServices;

namespace app.Services.Extension;

public struct ExtensionContext
{
    public ExtensionAssemblyLoadContext? LoadContext { get; private set; }
    public WeakReference WeakRef { get; private set; }

    public ExtensionContext(ExtensionAssemblyLoadContext loadContext)
    {
        LoadContext = loadContext;
        WeakRef = new(loadContext);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    public void Unload()
    {
        LoadContext?.Unload();
        LoadContext = null;

        for (var i = 0; WeakRef.IsAlive && i < 10; i++)
        {
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        Console.WriteLine($"Context IsAlive: {WeakRef.IsAlive}");
    }
}
