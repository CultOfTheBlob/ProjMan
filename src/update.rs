use std::{path::PathBuf, sync::Arc, thread::sleep, time::Duration};

use iced::{Task, clipboard};

use crate::{
    error::{Error, ErrorInfo},
    message::Message,
    project::{Project, project_creator},
    state::app_state::{AppState, NotifKind, Popup, ProjectCreationStatus},
    templates::{Templates, template::Template},
};

pub fn update(state: &mut AppState, message: Message) -> Task<Message>
{
    match message
    {
        Message::Tick =>
        {
            match state.load_projects()
            {
                Ok(projects) => state.project_list = Arc::new(projects),
                Err(err) => state.push_notification(err.get_message(), NotifKind::Error),
            };

            match Templates::generate()
            {
                Ok(templates) => state.templates = templates,
                Err(err) => state.push_notification(err.get_message(), NotifKind::Error),
            }

            Task::none()
        }
        Message::Opened(index) =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            if let Err(err) = state.project_list[index].run()
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
                project_creator::create_project_dir(Arc::clone(&state.new_project)),
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
                    project_creator::clone_project_repo(Arc::clone(&state.new_project)),
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
                    project_creator::create_projman_file(Arc::clone(&state.new_project)),
                    Message::ProjmanFileCreated,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::ProjmanFileCreated(result) => match result
        {
            Ok(log) =>
            {
                state.project_creation_status.log.push(log);

                state.project_creation_status.step += 1.0;

                sleep(Duration::from_millis(100));

                Task::perform(
                    project_creator::create_dir_structure(Arc::clone(&state.new_project)),
                    Message::DirStructureCreated,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::DirStructureCreated(result) => match result
        {
            Ok(log) =>
            {
                state.project_creation_status.log.push(log);

                state.project_creation_status.step += 1.0;

                sleep(Duration::from_millis(100));

                Task::perform(
                    project_creator::create_project_files(Arc::clone(&state.new_project)),
                    Message::ProjectFilesCreated,
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::ProjectFilesCreated(result) => match result
        {
            Ok(log) =>
            {
                state.project_creation_status.log.push(log);

                state.project_creation_status.step += 1.0;

                sleep(Duration::from_millis(100));

                Task::perform(
                    project_creator::execute_build_command(Arc::clone(&state.new_project), 0),
                    |result: Result<String, Error>| Message::BuildCommandExecuted(0, result),
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::BuildCommandExecuted(index, result) => match result
        {
            Ok(log) =>
            {
                state.project_creation_status.log.push(log);

                if index >= state.new_project.template.config().build.len() - 1
                {
                    state
                        .project_creation_status
                        .log
                        .push(String::from("Executed build commands..."));

                    state.project_creation_status.step += 1.0;

                    sleep(Duration::from_millis(100));

                    return Task::perform(
                        project_creator::commit_projman_init(Arc::clone(&state.new_project)),
                        Message::CommitedProjmanInit,
                    );
                }

                Task::perform(
                    project_creator::execute_build_command(
                        Arc::clone(&state.new_project),
                        index + 1,
                    ),
                    move |result: Result<String, Error>| {
                        Message::BuildCommandExecuted(index + 1, result)
                    },
                )
            }
            Err(err) => Task::perform(async { Err(err) }, Message::CreateFinished),
        },
        Message::CommitedProjmanInit(result) => match result
        {
            Ok(log) =>
            {
                state.project_creation_status.log.push(log);

                state.project_creation_status.step += 1.0;

                sleep(Duration::from_millis(100));

                Task::perform(
                    project_creator::add_project_to_json(Arc::clone(&state.new_project)),
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
                state.project_list = Arc::new(project_list);

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
                    state.new_project = Arc::new(Project::default(&state.config));
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
            state.new_project = Arc::new(Project::default(&state.config));
            state.pending = None;
            state.new_project_path_changed = false;

            Task::none()
        }
        Message::NewProjectNameChanged(name) =>
        {
            let project = Arc::make_mut(&mut state.new_project);

            if !state.new_project_path_changed
            {
                project.path = PathBuf::from(&state.config.general.projects_dir).join(&name);
            }
            project.name = name;
            Task::none()
        }
        Message::NewProjectTemplateChanged(template_name) =>
        {
            let project = Arc::make_mut(&mut state.new_project);

            let template: Arc<Template> = match state.templates.get(&template_name)
            {
                Ok(template) => template,
                Err(err) =>
                {
                    state.push_notification(err.get_message(), NotifKind::Warning);
                    return Task::none();
                }
            };

            project.template = template;

            project.template_name = template_name;

            Task::none()
        }
        Message::NewProjectRepoChanged(repo) =>
        {
            let project = Arc::make_mut(&mut state.new_project);

            project.repo = repo;

            Task::none()
        }
        Message::NewProjectPathChanged(path) =>
        {
            let project = Arc::make_mut(&mut state.new_project);

            project.path = path;

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
                }
                Err(err) => state.push_notification(err.get_message(), NotifKind::Error),
            }

            Task::none()
        }
        Message::ImportCanceled =>
        {
            state.pending = None;
            state.import_project_path = String::new();

            Task::none()
        }
        Message::ImportProjectPathChanged(path) =>
        {
            state.import_project_path = path;

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
                AppState::restore_project(state.selected_project, Arc::clone(&state.project_list)),
                Message::RemoveNonexistantFinished,
            )
        }
        Message::RemoveNonexistantFinished(restore_result) =>
        {
            state.restoring_project = false;

            match restore_result
            {
                Ok(index) => Arc::make_mut(&mut state.project_list)[index].exists = true,
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
