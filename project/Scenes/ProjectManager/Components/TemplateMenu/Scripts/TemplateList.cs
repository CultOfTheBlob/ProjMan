using Godot;
using Godot.Collections;
using System.IO;

public partial class TemplateList : ItemList
{
    public Dictionary<StringName, Template> Templates { get; set; } = new Dictionary<StringName, Template>();

    public override void _Ready()
    {
        if (!Config.IsValid()) Directory.CreateDirectory($"{Config.Path()}/templates");

        Template test = ResourceLoader.Load<Template>
        (
            "res://project/Scenes/ProjectManager/Components/TemplateMenu/Resources/Templates/TestTemplate.tres",
            "",
            ResourceLoader.CacheMode.Ignore
        );

        test.Name = "Billy";

        Error save = ResourceSaver.Save(test, "user://templates/test-template.tres");
        GD.Print($@"Error: {save}");

    }
}
