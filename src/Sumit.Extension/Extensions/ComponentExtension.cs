namespace Sumit.Extension.Extensions;

public static class ComponentExtension
{
    public static bool GetComponentEntry(this IPlugin plugin, out ComponentEntry entry)
    {
        var type = plugin.GetType();
        var attr = type.GetCustomAttributes(typeof(ComponentEntry<>), true).FirstOrDefault();
        
        if (attr is null)
        {
            entry = default!;
            return false;
        }
        
        var componentType = attr.GetType().GetGenericArguments()[0];
        entry = new ComponentEntry(componentType);
        
        return attr is not null;
    }
}