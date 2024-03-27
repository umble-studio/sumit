using System.Reflection;
using System.Runtime.Loader;

namespace app.Services.Extension;

public sealed class ExtensionAssemblyLoadContext() : AssemblyLoadContext(true)
{
    protected override Assembly? Load(AssemblyName assemblyName) => null;
}
