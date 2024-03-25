namespace Sumit.Extension;

public interface IPlugin
{
    string Name { get; }

    void OnLoad();
}