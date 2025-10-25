using Godot;

[GlobalClass]
public partial class Template : Resource
{
    [ExportGroup("BaseProperties")]

    [Export]
    public string Name { get; set; }


    public Template() : this(null) { }

    public Template(string name)
    {
        Name = name;
    }
}
