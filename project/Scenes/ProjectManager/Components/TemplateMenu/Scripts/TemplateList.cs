using Godot;
using Godot.Collections;
using System.IO;

public partial class TemplateList : ItemList
{
    [ExportGroup("Refrences")]

    [Export]
    public Label DescriptionLabel;


    public Dictionary<StringName, BaseTemplate> Templates { get; private set; } = new Dictionary<StringName, BaseTemplate>();


    /*
     * Handles checking if the templates folder exist.
     * All templates must be initialized here using the uid of an instance of the resource.
     * */
    public override void _Ready()
    {
        if (!Config.IsValid()) Directory.CreateDirectory($"{Config.Path()}/templates");

        Initialize<Template>("uid://dxt8d8n4npoli");

        foreach ((StringName templateName, BaseTemplate templateResource) in Templates)
        {
            this.AddItem(text: templateName, icon: templateResource.Icon);
        }
    }

    public void OnItemSelected(int index)
    {
        DescriptionLabel.Text = Templates[GetItemText(index)].Description;
    }


    /*
     * Loads the Resource inheriting BaseTemplate to the Templates Dictionary.
     * Saves the Resource into the config directory.
     * */
    private void Initialize<[MustBeVariant] T>(string uid) where T : BaseTemplate
    {
        T template = ResourceLoader.Load<T>(uid, cacheMode: ResourceLoader.CacheMode.Ignore);
        StringName name = template.Name ?? "None";

        Templates[template.Name ?? "None"] = template;

        if (!Config.IsValid())
        {
            GD.PushError("Config Directory Invalid.");
            return;
        }

        if (File.Exists($"{Config.Path()}/templates/{name}.tres")) return;

        Error save = Config.SaveTemplate(template, name);

        if (save != Error.Ok) GD.PushError(save);
    }
}
