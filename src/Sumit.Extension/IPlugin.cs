namespace Sumit.Extension;

public interface IPlugin
{
    string Name { get; }
    string TypeName { get; }

    void OnLoad();
}