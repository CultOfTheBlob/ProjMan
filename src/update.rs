use std::path::PathBuf;

use color_eyre::owo_colors::OwoColorize;
use iced::{Task, futures::TryFutureExt};

use crate::{
    message::Message,
    state::{
        app_state::{AppState, Popup},
        project::Project,
    },
    templates::{Command, TemplateConfig},
};

pub fn update(state: &mut AppState, message: Message) -> Task<Message>
{
    match AppState::create_project_list_from_json()
    {
        Ok(projects) => state.project_list = projects,
        Err(err) => eprintln!("{}", err.to_string().red()),
    };

    match message
    {
        Message::Open(project) =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            match project.run()
            {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err.red()),
            };

            Task::none()
        }
        Message::Select(index) =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            state.selected_project = Some(index);

            Task::none()
        }
        Message::Deselect =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            state.selected_project = None;

            Task::none()
        }
        Message::Remove =>
        {
            state.pending = Some(Popup::Remove);

            Task::none()
        }
        Message::ConfirmRemove =>
        {
            if let Some(Popup::Remove) = state.pending
            {
                match state.remove_project()
                {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", err.red()),
                }
            }

            state.selected_project = None;
            state.pending = None;
            state.delete_project_folder = state.config.general.delete_project_folder;

            Task::none()
        }
        Message::CancelRemove =>
        {
            state.pending = None;
            state.delete_project_folder = state.config.general.delete_project_folder;

            Task::none()
        }
        Message::ToggleRemoveProjectFolder(delete_project_folder) =>
        {
            state.delete_project_folder = delete_project_folder;

            Task::none()
        }
        Message::Create =>
        {
            state.pending = Some(Popup::Create);

            Task::none()
        }
        Message::ConfirmCreate =>
        {
            if !state
                .new_project
                .path_is_valid(&state.config.general.projects_dir)
                .0
            {
                return Task::none();
            }

            state.project_creation_status = (true, String::from("Creating project..."));

            Task::perform(
                AppState::create_project_dir(state.new_project.path.clone()),
                Message::CreateProjectDir,
            )
        }
        Message::FinishCreate(result) =>
        {
            match result
            {
                Ok(projects_list) =>
                {
                    state.project_creation_status.0 = false;
                    state
                        .project_creation_status
                        .1
                        .push_str("\nProject Created...");
                    state.project_list = projects_list;
                    state.new_project = Project::default(&state.config);
                    state.selected_project = Some(state.project_list.len() - 1);
                    // state.pending = None;
                    state.new_project_path_changed = false;
                }
                Err(err) =>
                {
                    state.project_creation_status.0 = false;
                    state
                        .project_creation_status
                        .1
                        .push_str(&format!("\nError: {err}"));
                }
            }

            Task::none()
        }
        Message::CreateProjectDir(result) => match result
        {
            Ok(msg) =>
            {
                state
                    .project_creation_status
                    .1
                    .push_str(&format!("\n{msg}"));

                Task::perform(
                    AppState::clone_project_repo(state.new_project.clone()),
                    Message::CloneProjectRepo,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::FinishCreate),
        },
        Message::CloneProjectRepo(result) => match result
        {
            Ok(msg) =>
            {
                state
                    .project_creation_status
                    .1
                    .push_str(&format!("\n{msg}"));

                Task::perform(
                    AppState::create_projman_file(state.new_project.path.clone()),
                    Message::CreateProjmanFile,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::FinishCreate),
        },
        Message::CreateProjmanFile(result) =>
        {
            let project_template: TemplateConfig = match state.new_project.project_type.template()
            {
                Ok(template) => template,
                Err(err) =>
                {
                    return Task::perform(
                        async move { Err(format!("Could not get project template ({err})")) },
                        Message::FinishCreate,
                    );
                }
            };

            match result
            {
                Ok(msg) =>
                {
                    state
                        .project_creation_status
                        .1
                        .push_str(&format!("\n{msg}"));

                    Task::perform(
                        AppState::create_dir_structure(
                            project_template.dir_structure,
                            state.new_project.path.clone(),
                        ),
                        Message::CreateDirStructure,
                    )
                }
                Err(err) => Task::perform(async { Err(err) }, Message::FinishCreate),
            }
        }
        Message::CreateDirStructure(result) =>
        {
            let project_template: TemplateConfig = match state.new_project.project_type.template()
            {
                Ok(template) => template,
                Err(err) =>
                {
                    return Task::perform(
                        async move { Err(format!("Could not get project template ({err})")) },
                        Message::FinishCreate,
                    );
                }
            };

            match result
            {
                Ok(msg) =>
                {
                    state
                        .project_creation_status
                        .1
                        .push_str(&format!("\n{msg}"));

                    Task::perform(
                        AppState::create_project_files(
                            project_template.files,
                            state.new_project.path.clone(),
                        ),
                        Message::CreateProjectFiles,
                    )
                }
                Err(err) => Task::perform(async { Err(err) }, Message::FinishCreate),
            }
        }
        Message::CreateProjectFiles(result) =>
        {
            let project_template: TemplateConfig = match state.new_project.project_type.template()
            {
                Ok(template) => template,
                Err(err) =>
                {
                    return Task::perform(
                        async move { Err(format!("Could not get project template ({err})")) },
                        Message::FinishCreate,
                    );
                }
            };

            match result
            {
                Ok(msg) =>
                {
                    state
                        .project_creation_status
                        .1
                        .push_str(&format!("\n{msg}"));

                    Task::perform(
                        AppState::execute_build_command(
                            project_template.build[0].clone(),
                            state.new_project.path.clone(),
                        ),
                        |result: Result<String, String>| Message::ExecuteBuildCommand(0, result),
                    )
                }
                Err(err) => Task::perform(async { Err(err) }, Message::FinishCreate),
            }
        }
        Message::ExecuteBuildCommand(index, result) =>
        {
            if let Ok(project_template) = state.new_project.project_type.template()
            {
                let commands: Vec<Command> = project_template.build;

                match result
                {
                    Ok(msg) =>
                    {
                        state
                            .project_creation_status
                            .1
                            .push_str(&format!("\n{msg}"));

                        if index >= commands.len() - 1
                        {
                            state
                                .project_creation_status
                                .1
                                .push_str("\nExecuted build commands...");

                            return Task::perform(
                                AppState::add_project_to_json(state.new_project.clone()),
                                Message::FinishCreate,
                            );
                        }

                        return Task::perform(
                            AppState::execute_build_command(
                                commands[index + 1].clone(),
                                state.new_project.path.clone(),
                            ),
                            move |result: Result<String, String>| {
                                Message::ExecuteBuildCommand(index + 1, result)
                            },
                        );
                    }
                    Err(err) => return Task::perform(async { Err(err) }, Message::FinishCreate),
                }
            };

            Task::none()
        }
        Message::CancelCreate =>
        {
            state.project_creation_status = (false, String::new());
            state.new_project = Project::default(&state.config);
            state.pending = None;
            state.new_project_path_changed = false;

            Task::none()
        }
        Message::ChangeNewProjectName(name) =>
        {
            if !state.new_project_path_changed
            {
                state.new_project.path =
                    PathBuf::from(&state.config.general.projects_dir).join(&name);
            }

            state.new_project.name = name;

            Task::none()
        }
        Message::ChangeNewProjectType(project_type) =>
        {
            state.new_project.project_type = project_type;

            Task::none()
        }
        Message::ChangeNewProjectRepo(repo) =>
        {
            state.new_project.repo = repo;

            Task::none()
        }
        Message::ChangeNewProjectPath(path) =>
        {
            state.new_project.path = path;

            state.new_project_path_changed = true;

            Task::none()
        }
        Message::Import => Task::none(),
        Message::RemoveNonexistant =>
        {
            match state.remove_project()
            {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err.red()),
            }

            state.selected_project = None;
            Task::none()
        }
        Message::RestoreNonexistant =>
        {
            state.project_restoration_failed = false;
            state.restoring_project = true;

            Task::perform(
                AppState::restore_project(state.selected_project, state.project_list.clone())
                    .map_err(|e| e.to_string()),
                Message::FinishRemoveNonexistant,
            )
        }
        Message::FinishRemoveNonexistant(restore_result) =>
        {
            state.restoring_project = false;

            match restore_result
            {
                Ok(index) => state.project_list[index].exists = true,
                Err(_) => state.project_restoration_failed = true,
            }

            Task::none()
        }
    }
}
