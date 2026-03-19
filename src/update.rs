use std::path::PathBuf;

use color_eyre::owo_colors::OwoColorize;
use iced::{Task, clipboard, futures::TryFutureExt};

use crate::{
    error::Error,
    message::Message,
    project::{Project, project_creator},
    state::app_state::{AppState, Popup, ProjectCreationStatus},
    templates::{Command, TemplateConfig},
};

pub fn update(state: &mut AppState, message: Message) -> Task<Message>
{
    match AppState::create_project_list_from_json()
    {
        Ok(projects) => state.project_list = projects,
        Err(err) => eprintln!("{}", err.get_message().red()),
    };

    match message
    {
        Message::Opened(project) =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            if let Err(err) = project.run()
            {
                eprintln!("{}", err.get_message().red());
            }

            Task::none()
        }
        Message::Selected(index) =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            state.selected_project = Some(index);

            Task::none()
        }
        Message::Deselected =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            state.selected_project = None;

            Task::none()
        }
        Message::Removed =>
        {
            state.pending = Some(Popup::Remove);

            Task::none()
        }
        Message::RemoveConfirmed =>
        {
            if let Some(Popup::Remove) = state.pending
                && let Err(err) = state.remove_project()
            {
                eprintln!("{}", err.get_message().red())
            }

            state.selected_project = None;
            state.pending = None;
            state.delete_project_folder = state.config.general.delete_project_folder;

            Task::none()
        }
        Message::RemoveCanceled =>
        {
            state.pending = None;
            state.delete_project_folder = state.config.general.delete_project_folder;

            Task::none()
        }
        Message::RemoveProjectFolderToggled(delete_project_folder) =>
        {
            state.delete_project_folder = delete_project_folder;

            Task::none()
        }
        Message::Created =>
        {
            state.pending = Some(Popup::Create);

            Task::none()
        }
        Message::CreateConfirmed =>
        {
            if state
                .new_project
                .path_is_valid(&state.config.general.projects_dir)
                .is_err()
            {
                return Task::none();
            }

            state.project_creation_status = ProjectCreationStatus {
                creating: true,
                failed: false,
                log: vec![String::from("Creating project...")],
            };

            Task::perform(
                project_creator::create_project_dir(state.new_project.path.clone()),
                Message::ProjectDirCreated,
            )
        }
        Message::ProjectDirCreated(result) => match result
        {
            Ok(log) =>
            {
                state.project_creation_status.log.push(log);

                Task::perform(
                    project_creator::clone_project_repo(state.new_project.clone()),
                    Message::ProjectRepoCloned,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::ProjectRepoCloned(result) => match result
        {
            Ok(log) =>
            {
                state.project_creation_status.log.push(log);

                Task::perform(
                    project_creator::create_projman_file(
                        state.new_project.path.clone(),
                        state.new_project.project_type.clone(),
                    ),
                    Message::ProjmanFileCreated,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::ProjmanFileCreated(result) =>
        {
            let project_template: TemplateConfig = match state.new_project.project_type.template()
            {
                Ok(template) => template,
                Err(err) =>
                {
                    return Task::perform(async move { Err(err) }, Message::CreateFinished);
                }
            };

            match result
            {
                Ok(log) =>
                {
                    state.project_creation_status.log.push(log);

                    Task::perform(
                        project_creator::create_dir_structure(
                            project_template.dir_structure,
                            state.new_project.path.clone(),
                        ),
                        Message::DirStructureCreated,
                    )
                }
                Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
            }
        }
        Message::DirStructureCreated(result) =>
        {
            let project_template: TemplateConfig = match state.new_project.project_type.template()
            {
                Ok(template) => template,
                Err(err) =>
                {
                    return Task::perform(async move { Err(err) }, Message::CreateFinished);
                }
            };

            match result
            {
                Ok(log) =>
                {
                    state.project_creation_status.log.push(log);

                    Task::perform(
                        project_creator::create_project_files(
                            project_template.files,
                            state.new_project.path.clone(),
                        ),
                        Message::ProjectFilesCreated,
                    )
                }
                Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
            }
        }
        Message::ProjectFilesCreated(result) =>
        {
            let project_template: TemplateConfig = match state.new_project.project_type.template()
            {
                Ok(template) => template,
                Err(err) =>
                {
                    return Task::perform(async move { Err(err) }, Message::CreateFinished);
                }
            };

            match result
            {
                Ok(log) =>
                {
                    state.project_creation_status.log.push(log);

                    Task::perform(
                        project_creator::execute_build_command(
                            project_template.build[0].clone(),
                            state.new_project.path.clone(),
                        ),
                        |result: Result<String, Error>| Message::BuildCommandExecuted(0, result),
                    )
                }
                Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
            }
        }
        Message::BuildCommandExecuted(index, result) =>
        {
            if let Ok(project_template) = state.new_project.project_type.template()
            {
                let commands: Vec<Command> = project_template.build;

                match result
                {
                    Ok(log) =>
                    {
                        state.project_creation_status.log.push(log);

                        if index >= commands.len() - 1
                        {
                            state
                                .project_creation_status
                                .log
                                .push(String::from("Executed build commands..."));

                            return Task::perform(
                                project_creator::commit_projman_init(state.new_project.clone()),
                                Message::CommitedProjmanInit,
                            );
                        }

                        return Task::perform(
                            project_creator::execute_build_command(
                                commands[index + 1].clone(),
                                state.new_project.path.clone(),
                            ),
                            move |result: Result<String, Error>| {
                                Message::BuildCommandExecuted(index + 1, result)
                            },
                        );
                    }
                    Err(err) => return Task::perform(async { Err(err) }, Message::CreateFinished),
                }
            };

            Task::none()
        }
        Message::CommitedProjmanInit(result) => match result
        {
            Ok(log) =>
            {
                state.project_creation_status.log.push(log);

                Task::perform(
                    project_creator::add_project_to_json(state.new_project.clone()),
                    Message::CreateFinished,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::CreateFinished(result) =>
        {
            match result
            {
                Ok(project_list) =>
                {
                    state.project_creation_status.creating = false;
                    state
                        .project_creation_status
                        .log
                        .push(String::from("Project Created!"));
                    state.project_list = project_list;
                    state.new_project = Project::default(&state.config);
                    state.selected_project = Some(state.project_list.len() - 1);
                    state.pending = None;
                    state.new_project_path_changed = false;
                }
                Err(err) =>
                {
                    state.project_creation_status.creating = false;
                    state.project_creation_status.failed = true;
                    state.project_creation_status.log.push(err.get_message());
                }
            }

            Task::none()
        }
        Message::CreateCanceled =>
        {
            state.project_creation_status = ProjectCreationStatus {
                creating: false,
                failed: false,
                log: vec![String::new()],
            };
            state.new_project = Project::default(&state.config);
            state.pending = None;
            state.new_project_path_changed = false;

            Task::none()
        }
        Message::NewProjectNameChanged(name) =>
        {
            if !state.new_project_path_changed
            {
                state.new_project.path =
                    PathBuf::from(&state.config.general.projects_dir).join(&name);
            }

            state.new_project.name = name;

            Task::none()
        }
        Message::NewProjectTypeChanged(project_type) =>
        {
            state.new_project.project_type = project_type;

            Task::none()
        }
        Message::NewProjectRepoChanged(repo) =>
        {
            state.new_project.repo = repo;

            Task::none()
        }
        Message::NewProjectPathChanged(path) =>
        {
            state.new_project.path = path;

            state.new_project_path_changed = true;

            Task::none()
        }
        Message::CreationErrorCopied =>
        {
            if let Some(error) = state.project_creation_status.log.last()
            {
                return clipboard::write::<Message>(error.to_string());
            }

            Task::none()
        }
        Message::Imported =>
        {
            state.pending = Some(Popup::Import);

            Task::none()
        }
        Message::ImportConfirmed =>
        {
            if let Err(err) = state.import_project()
            {
                eprint!("{}", err.get_message().red())
            }

            state.pending = None;
            state.import_project_path = String::new();
            state.import_project_name = String::new();
            state.import_project_name_changed = false;

            Task::none()
        }
        Message::ImportCanceled =>
        {
            state.pending = None;
            state.import_project_path = String::new();
            state.import_project_name = String::new();
            state.import_project_name_changed = false;

            Task::none()
        }
        Message::ImportProjectPathChanged(path) =>
        {
            if state.import_project_name_changed
            {
                state.import_project_path = path;

                return Task::none();
            }

            if let Some(last) = PathBuf::from(&path).iter().next_back()
            {
                state.import_project_name = last.to_string_lossy().to_string()
            }

            state.import_project_path = path;

            Task::none()
        }
        Message::ImportProjectNameChanged(name) =>
        {
            state.import_project_name = name;
            state.import_project_name_changed = true;

            Task::none()
        }
        Message::NonexistantRemoved =>
        {
            if let Err(err) = state.remove_project()
            {
                eprintln!("{}", err.get_message().red())
            }

            state.selected_project = None;
            Task::none()
        }
        Message::NonexistantRestored =>
        {
            state.project_restoration_failed = false;
            state.restoring_project = true;

            Task::perform(
                AppState::restore_project(state.selected_project, state.project_list.clone())
                    .map_err(|e| e.get_message()),
                Message::RemoveNonexistantFinished,
            )
        }
        Message::RemoveNonexistantFinished(restore_result) =>
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
