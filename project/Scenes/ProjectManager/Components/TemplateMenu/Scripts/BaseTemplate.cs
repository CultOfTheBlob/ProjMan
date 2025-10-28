using Godot;

[GlobalClass]
public abstract partial class BaseTemplate : Resource
{
    public abstract StringName Name { get; }

    public abstract string Description { get; }

    public abstract Texture2D Icon { get; }

    public abstract string Flake { get; set; }
}
