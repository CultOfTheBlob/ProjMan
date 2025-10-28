using Godot;

[GlobalClass]
public partial class Template : BaseTemplate
{
    public override StringName Name { get; } = "Template";

    public override string Description { get; } =
    @"
        This is a template.
        uhh... idk
    ";

    public override Texture2D Icon { get; } = ResourceLoader.Load<Texture2D>("uid://e5ejfya5588l");


    [ExportGroup("BaseProperties")]

    [Export]
    public override string Flake { get; set; }


    public Template() : this(null) { }

    public Template(string flake)
    {
        Flake = flake;
    }
}
