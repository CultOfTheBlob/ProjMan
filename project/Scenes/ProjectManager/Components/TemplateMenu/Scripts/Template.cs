using Godot;

[GlobalClass]
public partial class Template : BaseTemplate
{
    private string flake = @"";


    public override StringName Name { get; } = "Template";

    public override string Description { get; } =
    @"
        This is a template.
        uhh... idk
    ";

    public override Texture2D Icon { get; } = ResourceLoader.Load<Texture2D>("uid://e5ejfya5588l");


    [ExportGroup("BaseProperties")]

    [Export]
    public override string Flake
    {
        get => flake;
        set
        {
            flake = value;
            Config.SaveTemplate(this, Name);
        }
    }


    public Template() : this(null) { }

    public Template(string flake)
    {
        this.flake = flake;
    }
}
