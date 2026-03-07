use crate::templates::{Command, Template, TemplateConfig};

#[derive(Debug)]
pub struct Base;

impl Template for Base
{
    fn default() -> TemplateConfig
    {
        TemplateConfig {
            dir_structure: vec![
                Folder {
                    name: String::from("src"),
                    sub_dirs: vec![Folder {
                        name: String::from("utils"),
                        sub_dirs: vec![],
                    }],
                },
                Folder {
                    name: String::from("bin"),
                    sub_dirs: vec![],
                },
            ],

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
