namespace Sumit.Extension;

public abstract class Extension
{
    public ExtensionManifest Manifest { get; private set; }
    public bool IsEnabled { get; set; }

    internal void __ctor(ExtensionManifest manifest)
    {
        Manifest = manifest;
    }
    
    public virtual void OnLoad()
    {
    }
    
    public virtual void OnReload()
    {
    }

    public virtual void OnEnabled()
    {
        IsEnabled = true;
    }

    public virtual void OnDisabled()
    {
        IsEnabled = false;
    }
}