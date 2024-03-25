using Microsoft.AspNetCore.Components;

namespace Sumit.Extension;

[AttributeUsage(AttributeTargets.Class)]
public sealed class ComponentEntry<T> : Attribute where T : IComponent
{
    public Type Type { get; } = typeof(T);
}

public sealed class ComponentEntry(Type type)
{
    public Type Type { get; } = type;
}