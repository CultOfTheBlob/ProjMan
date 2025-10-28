using Godot;
using System.IO;
using System.Linq;

public partial class Config : Node
{
    private static string[] requiredDirectories = ["/templates"];

    public static string Path() => $"{OS.GetDataDir()}/projman";

    public static bool IsValid() => requiredDirectories.All(path => Directory.Exists($"{Config.Path()}{path}"));

    public static Error SaveTemplate(BaseTemplate template, string name) =>
        ResourceSaver.Save(template, $"user://templates/{name}.tres");
}
