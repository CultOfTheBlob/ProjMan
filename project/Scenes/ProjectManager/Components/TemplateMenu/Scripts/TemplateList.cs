using Godot;
using Godot.Collections;

public partial class TemplateList : ItemList
{
    [Export]
    public Dictionary<StringName, Template> Templates { get; set; } = new Dictionary<StringName, Template>();
}
