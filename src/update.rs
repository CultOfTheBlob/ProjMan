use std::{path::PathBuf, thread::sleep, time::Duration};

use iced::{Task, clipboard};

use crate::{
    error::{Error, ErrorInfo},
    message::Message,
    project::{Project, project_creator},
    state::app_state::{AppState, NotifKind, Popup, ProjectCreationStatus},
    templates::{template_config::Command, template_config::TemplateConfig},
};

pub fn update(state: &mut AppState, message: Message) -> Task<Message>
{
    match message
    {
        Message::Tick =>
        {
            match AppState::create_project_list_from_json()
            {
                Ok(projects) => state.project_list = projects,
                Err(err) => state.push_notification(err.get_message(), NotifKind::Error),
            };

            Task::none()
        }
        Message::Opened(project) =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            if let Err(err) = project.run()
            {
                state.push_notification(err.get_message(), NotifKind::Error);
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
        Message::Updated =>
        {
            if let Err(err) = state.update_project()
            {
                state.push_notification(err.get_message(), NotifKind::Error)
            }

            Task::none()
        }
        Message::Edited(index) =>
        {
            state.pending = Some(Popup::Edit);

            let project: &Project = &state.project_list[index];

            state.edit_project_name = project.name.to_string();
            state.edit_project_repo = project.repo.to_string();

            Task::none()
        }
        Message::EditConfirmed =>
        {
            if let Some(Popup::Edit) = state.pending
            {
                match state.edit_project()
                {
                    Ok(_) =>
                    {
                        state.pending = None;
                    }
                    Err(err) => state.push_notification(err.get_message(), NotifKind::Error),
                }
            }

            Task::none()
        }
        Message::EditCanceled =>
        {
            state.pending = None;

            Task::none()
        }
        Message::EditProjectNameChanged(name) =>
        {
            state.edit_project_name = name;

            Task::none()
        }
        Message::EditProjectRepoChanged(repo) =>
        {
            state.edit_project_repo = repo;

            Task::none()
        }
        Message::RepoOpened =>
        {
            if let Some(index) = state.selected_project
            {
                let project: &Project = &state.project_list[index];

                if let Err(err) = open::that(&project.repo)
                {
                    state.push_notification(
                        error!(Error::Open, "project repo", err).get_message(),
                        NotifKind::Error,
                    );
                }
            }

            Task::none()
        }
        Message::SideBarToggled =>
        {
            state.sidebar_expanded = !state.sidebar_expanded;

            Task::none()
        }
        Message::NotificationRemoved(index) =>
        {
            state.notifications.remove(index);

            Task::none()
        }
        Message::NotificationCopied(index) =>
        {
            clipboard::write::<Message>(state.notifications[index].text.to_string())
        }
        Message::Removed =>
        {
            state.pending = Some(Popup::Remove);

            Task::none()
        }
        Message::RemoveConfirmed =>
        {
            if let Some(Popup::Remove) = state.pending
            {
                match state.remove_project()
                {
                    Ok(_) =>
                    {
                        state.selected_project = None;
                        state.pending = None;
                        state.delete_project_folder = state.config.general.delete_project_folder;
                    }
                    Err(err) => state.push_notification(err.get_message(), NotifKind::Error),
                }
            }

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
                step: 0.0,
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

                state.project_creation_status.step += 1.0;

                sleep(Duration::from_millis(100));

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

                state.project_creation_status.step += 1.0;

                sleep(Duration::from_millis(100));

                Task::perform(
                    project_creator::create_projman_file(
                        state.new_project.path.clone(),
                        state.new_project.template.clone(),
                    ),
                    Message::ProjmanFileCreated,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::ProjmanFileCreated(result) =>
        {
            let project_template: &TemplateConfig = state.new_project.template.config();

            match result
            {
                Ok(log) =>
                {
                    state.project_creation_status.log.push(log);

                    state.project_creation_status.step += 1.0;

                    sleep(Duration::from_millis(100));

                    Task::perform(
                        project_creator::create_dir_structure(
                            project_template.dir_structure.clone(),
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
            let project_template: &TemplateConfig = state.new_project.template.config();

            match result
            {
                Ok(log) =>
                {
                    state.project_creation_status.log.push(log);

                    state.project_creation_status.step += 1.0;

                    sleep(Duration::from_millis(100));

                    Task::perform(
                        project_creator::create_project_files(
                            project_template.files.clone(),
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
            let project_template: &TemplateConfig = state.new_project.template.config();

            match result
            {
                Ok(log) =>
                {
                    state.project_creation_status.log.push(log);

                    state.project_creation_status.step += 1.0;

                    sleep(Duration::from_millis(100));

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
            let commands: &Vec<Command> = &state.new_project.template.config().build;

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

                        state.project_creation_status.step += 1.0;

                        sleep(Duration::from_millis(100));

                        return Task::perform(
                            project_creator::commit_projman_init(state.new_project.clone()),
                            Message::CommitedProjmanInit,
                        );
                    }

                    Task::perform(
                        project_creator::execute_build_command(
                            commands[index + 1].clone(),
                            state.new_project.path.clone(),
                        ),
                        move |result: Result<String, Error>| {
                            Message::BuildCommandExecuted(index + 1, result)
                        },
                    )
                }
                Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
            }
        }
        Message::CommitedProjmanInit(result) => match result
        {
            Ok(log) =>
            {
                state.project_creation_status.log.push(log);

                state.project_creation_status.step += 1.0;

                sleep(Duration::from_millis(100));

                Task::perform(
                    project_creator::add_project_to_json(state.new_project.clone()),
                    Message::ProjectAddedToJson,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::ProjectAddedToJson(result) => match result
        {
            Ok(project_list) =>
            {
                state
                    .project_creation_status
                    .log
                    .push(String::from("Project Created!"));
                state.project_creation_status.step += 1.0;
                state.project_list = project_list;

                sleep(Duration::from_millis(1000));

                Task::perform(async { Ok(()) }, Message::CreateFinished)
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::CreateFinished(result) =>
        {
            match result
            {
                Ok(_) =>
                {
                    state.project_creation_status = ProjectCreationStatus {
                        creating: false,
                        failed: false,
                        step: 0.0,
                        log: vec![String::new()],
                    };
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
                step: 0.0,
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
            state.new_project.template = project_type;

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
            match state.import_project()
            {
                Ok(_) =>
                {
                    state.pending = None;
                    state.import_project_path = String::new();
                    state.import_project_name = String::new();
                    state.import_project_name_changed = false;
                }
                Err(err) => state.push_notification(err.get_message(), NotifKind::Error),
            }

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
                state.push_notification(err.get_message(), NotifKind::Error)
            }

            state.selected_project = None;
            Task::none()
        }
        Message::NonexistantRestored =>
        {
            state.project_restoration_failed = false;
            state.restoring_project = true;

            Task::perform(
                AppState::restore_project(state.selected_project, state.project_list.clone()),
                Message::RemoveNonexistantFinished,
            )
        }
        Message::RemoveNonexistantFinished(restore_result) =>
        {
            state.restoring_project = false;

            match restore_result
            {
                Ok(index) => state.project_list[index].exists = true,
                Err(err) =>
                {
                    state.project_restoration_failed = true;

                    state.push_notification(err.get_message(), NotifKind::Error);
                }
            }

            Task::none()
        }
    }
}
