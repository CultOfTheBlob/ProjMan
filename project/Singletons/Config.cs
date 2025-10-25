using Godot;
using System.IO;
using System.Linq;

public partial class Config : Node
{
    private static string[] requiredDirectories = ["/template"];

    public static string Path() => $"{OS.GetDataDir()}/projman";

    public static bool IsValid() => requiredDirectories.All(path => Directory.Exists($"{Config.Path}{path}"));
}
