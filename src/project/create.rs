use crate::{
    app_state::AppState,
    prelude::*,
    project::{Existant, Nonexistant, Project},
    template::{command::Command, file::File, folder::Folder},
};
use askalono::{Store, TextData};
use futures::AsyncBufReadExt;
use futures::StreamExt;
use futures::io::BufReader;
use git2::{IndexAddOption, Repository};
use smol::process::{Command as AsyncCommand, Stdio};
use std::{
    fs::{self},
    iter,
    marker::PhantomData,
    sync::Arc,
};

const CARET_COLOR: &str = "\x1b[1;35m";
const COMMAND_COLOR: &str = "\x1b[1;36m";
const RESET_COLOR: &str = "\x1b[0m";

impl Project<Nonexistant> {
    pub async fn create<F>(
        mut self,
        app_state: &AppState,
        on_log: F,
    ) -> Result<Project<Existant>>
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let on_log = Arc::new(on_log);

        let template = self.get_template(app_state)?;

        create_project_dir(&self, &on_log)?;

        clone_project_repo(&self, &on_log)?;

        load_project_license(&mut self)?;

        create_project_file(&self, &on_log)?;

        create_project_dir_structure(&self, &template.config.dir_structure, &on_log)?;

        create_project_files(&self, &template.config.files, &on_log)?;

        execute_build_commands(&self, &template.config.build, &on_log).await?;

        commit_project_init(&self, &on_log)?;

        Ok(Project::<Existant> {
            name: self.name,
            path: self.path,
            repo: self.repo,
            license: self.license,
            template_name: self.template_name,
            state: PhantomData,
        })
    }
}

fn create_project_dir<F>(project: &Project<Nonexistant>, on_log: &Arc<F>) -> Result<()>
where
    F: Fn(String) + Send + Sync + 'static,
{
    on_log(format!(
        "{CARET_COLOR}>{RESET_COLOR} {COMMAND_COLOR}mkdir -p {}{RESET_COLOR}",
        project.path.display()
    ));

    fs::create_dir_all(&project.path)
        .map_err(|err| Error::CreateProjectDir(err.to_string()))
}

fn clone_project_repo<F>(project: &Project<Nonexistant>, on_log: &Arc<F>) -> Result<()>
where
    F: Fn(String) + Send + Sync + 'static,
{
    on_log(format!(
        "{CARET_COLOR}>{RESET_COLOR} {COMMAND_COLOR}git clone {} {}{RESET_COLOR}",
        project.repo,
        project.path.display()
    ));

    project.clone_repo()
}

fn load_project_license(project: &mut Project<Nonexistant>) -> Result<()> {
    let cache = &include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/cache/license.cache.zstd"
    ))[..];

    let store = Store::from_cache(cache)
        .map_err(|err| Error::LoadProjectLicense(err.to_string()))?;

    let license_path = project.path.join("LICENSE");

    let license_contents = fs::read_to_string(&license_path)
        .map_err(|err| Error::LoadProjectLicense(err.to_string()))?;

    let license = store
        .analyze(&TextData::from(license_contents.as_str()))
        .name
        .to_owned();

    project.license = license;

    Ok(())
}

fn create_project_file<F>(project: &Project<Nonexistant>, on_log: &Arc<F>) -> Result<()>
where
    F: Fn(String) + Send + Sync + 'static,
{
    let project_file_path = project.get_project_file_path();

    on_log(format!(
        "{CARET_COLOR}>{RESET_COLOR} {COMMAND_COLOR}touch {}{RESET_COLOR}",
        project_file_path.display()
    ));

    let project_file_contents = toml::to_string_pretty(&project)
        .map_err(|err| Error::CreateProjectFile(err.to_string()))?;

    fs::write(project_file_path, project_file_contents)
        .map_err(|err| Error::CreateProjectFile(err.to_string()))?;

    Ok(())
}

fn create_project_dir_structure<F>(
    project: &Project<Nonexistant>,
    dir_structure: &[Folder],
    on_log: &Arc<F>,
) -> Result<()>
where
    F: Fn(String) + Send + Sync + 'static,
{
    for folder in dir_structure {
        let dirs = folder.resolve(&project.path, &project.into());

        for dir in dirs {
            on_log(format!(
                "{CARET_COLOR}>{RESET_COLOR} {COMMAND_COLOR}touch {}{RESET_COLOR}",
                dir.display()
            ));

            fs::create_dir_all(&dir)
                .map_err(|err| Error::CreateProjectDirStructure(err.to_string()))?;
        }
    }

    Ok(())
}

fn create_project_files<F>(
    project: &Project<Nonexistant>,
    files: &[File],
    on_log: &Arc<F>,
) -> Result<()>
where
    F: Fn(String) + Send + Sync + 'static,
{
    for file in files {
        let file = file.resolve(&project.path, &project.into());

        on_log(format!(
            "{CARET_COLOR}>{RESET_COLOR} {COMMAND_COLOR}touch {}{RESET_COLOR}",
            file.path
        ));

        fs::write(file.path, file.contents)
            .map_err(|err| Error::CreateProjectFiles(err.to_string()))?;
    }

    Ok(())
}

async fn execute_build_commands<F>(
    project: &Project<Nonexistant>,
    commands: &[Command],
    on_log: &Arc<F>,
) -> Result<()>
where
    F: Fn(String) + Send + Sync + 'static,
{
    for command in commands {
        on_log(format!(
            "{CARET_COLOR}>{RESET_COLOR} {COMMAND_COLOR}{} {}{RESET_COLOR}",
            command.program,
            command.args.join(" ")
        ));

        let mut child = AsyncCommand::new(&command.program)
            .args(&command.args)
            .current_dir(&project.path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| Error::ExecuteProjectCommands(err.to_string()))?;

        let stdout = child.stdout.take();

        smol::spawn({
            let on_log = Arc::clone(on_log);

            async move {
                let Some(stdout) = stdout else {
                    return;
                };

                let mut lines = BufReader::new(stdout).lines();

                while let Some(Ok(line)) = lines.next().await {
                    on_log(line);
                }
            }
        })
        .await;

        let stderr = child.stderr.take();

        smol::spawn({
            let on_log = Arc::clone(on_log);

            async move {
                let Some(stderr) = stderr else {
                    return;
                };

                let mut lines = BufReader::new(stderr).lines();

                while let Some(Ok(line)) = lines.next().await {
                    on_log(line);
                }
            }
        })
        .await;

        let status = child
            .status()
            .await
            .map_err(|err| Error::ExecuteProjectCommands(err.to_string()))?;

        if !status.success() {
            return Err(Error::ExecuteProjectCommands(format!(
                "Command failed with exit code: {:?}",
                status.code()
            )));
        }
    }

    Ok(())
}

fn commit_project_init<F>(project: &Project<Nonexistant>, on_log: &Arc<F>) -> Result<()>
where
    F: Fn(String) + Send + Sync + 'static,
{
    on_log(String::from(
        "{CARET}>{RESET} {COMMAND}git add . && git commit -m{RESET}",
    ));

    let project_repo = Repository::open(&project.path)
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    let mut index = project_repo
        .index()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    index
        .add_all(iter::once(&"*"), IndexAddOption::DEFAULT, None)
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    index
        .write()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    let signature = project_repo
        .signature()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    let tree = project_repo
        .find_tree(
            index
                .write_tree()
                .map_err(|err| Error::CommitProjectInit(err.to_string()))?,
        )
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    let parent_commit = project_repo
        .head()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?
        .peel_to_commit()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    project_repo
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initialized ProjMan project",
            &tree,
            &[&parent_commit],
        )
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    Ok(())
}
