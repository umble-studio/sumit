namespace Sumit.Extension;

public abstract class Extension
{
    public abstract string Name { get; }
    public bool IsEnabled { get; private set; }
    
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