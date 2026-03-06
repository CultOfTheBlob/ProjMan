use crate::templates::{Command, Template, TemplateConfig};

#[derive(Debug)]
pub struct Base;

impl Template for Base
{
    fn default() -> TemplateConfig
    {
        TemplateConfig {
            dir_structure: None,

            run: vec![Command {
                program: String::from("kitty"),
                args: vec![String::from("--detach")],
            }],
        }
    }

    fn template_path() -> &'static str
    {
        "templates/base.json"
    }
}
