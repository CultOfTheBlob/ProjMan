#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to initialize GPUI window: {0}")]
    CreateWindow(String),
    #[error("Failed to open [{0}] popup: {1}")]
    OpenPopup(String, String),
    #[error("Failed to parse ProjMan config: {0}")]
    ParseConfig(String),
    #[error("Failed to create directory [{0}]: {1}")]
    CreateDir(String, String),
    #[error("ProjMan config is invalid: {0}")]
    ValidateConfig(String),
    #[error("Failed to get ProjMan config directory")]
    GetConfigDir,
    #[error("Failed to read template [{0}]: {1}")]
    ReadTemplate(String, String),
    #[error("Failed to parse template [{0}]: {1}")]
    ParseTemplate(String, String),
    #[error("Failed to run command [{0}]: {1}")]
    RunCommand(String, String),
    #[error("Failed to read templates directory: {0}")]
    ReadTemplatesDir(String),
    #[error("Failed to get template [{0}]")]
    GetTemplate(String),
    #[error("Failed to read project file: {0}")]
    ReadProjectFile(String),
    #[error("Failed to read project list: {0}")]
    ReadProjectList(String),
    #[error("Failed to get project information: {0}")]
    GetProjectInfo(String),
    #[error("Failed to update projects: {0}")]
    UpdateProjects(String),
    #[error("Failed to remove project from projects: {0}")]
    RemoveProject(String),
    #[error("Failed to restore project: {0}")]
    RestoreProject(String),
    #[error("Failed to clone project repo: {0}")]
    CloneProjectRepo(String),
    #[error("Failed to edit projects: {0}")]
    EditProjects(String),
    #[error("Failed to import projects: {0}")]
    ImportProjects(String),
    #[error("Failed to create project directory: {0}")]
    CreateProjectDir(String),
    #[error("Failed to create project file: {0}")]
    CreateProjectFile(String),
    #[error("Failed to create project directory structure: {0}")]
    CreateProjectDirStructure(String),
    #[error("Failed to create project files: {0}")]
    CreateProjectFiles(String),
    #[error("Failed to execute project commands: {0}")]
    ExecuteProjectCommands(String),
    #[error("Failed to execute commit project init: {0}")]
    CommitProjectInit(String),
    #[error("Failed to load project license: {0}")]
    LoadProjectLicense(String),
}
