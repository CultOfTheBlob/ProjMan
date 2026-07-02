macro_rules! error {
    ($type:path, $str:expr, $err:expr) => {
        $type(ErrorInfo::new($str.to_string(), $err.to_string()))
    };
}

#[derive(Debug, Clone)]
pub enum Error {
    Other(String),
    Create(ErrorInfo),
    Write(ErrorInfo),
    Find(ErrorInfo),
    Remove(ErrorInfo),
    Read(ErrorInfo),
    Parse(ErrorInfo),
    Clone(ErrorInfo),
    Run(ErrorInfo),
    Open(ErrorInfo),
    Commit(ErrorInfo),
}

impl Error {
    pub fn get_message(&self) -> String {
        match self {
            Self::Other(message) => format!("Error: {message}"),
            Self::Create(ErrorInfo { string, err }) => {
                format!("Error: Could not create {string} ({err})")
            }
            Self::Write(ErrorInfo { string, err }) => {
                format!("Error: Could not write to {string} ({err})")
            }
            Self::Find(ErrorInfo { string, err }) => {
                format!("Error: Could not find {string} ({err})")
            }
            Self::Remove(ErrorInfo { string, err }) => {
                format!("Error: Could not remove {string} file ({err})")
            }
            Self::Read(ErrorInfo { string, err }) => {
                format!("Error: Could not read {string} ({err})")
            }
            Self::Parse(ErrorInfo { string, err }) => {
                format!("Error: Could not parse {string} ({err})")
            }
            Self::Clone(ErrorInfo { string, err }) => {
                format!("Error: Could not clone {string} ({err})")
            }
            Self::Run(ErrorInfo { string, err }) => {
                format!("Error: Could not run [{string}] ({err})")
            }
            Self::Open(ErrorInfo { string, err }) => {
                format!("Error: Could not open {string} ({err})")
            }
            Self::Commit(ErrorInfo { string, err }) => {
                format!("Error: Could not commit {string} ({err})")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    string: String,
    err: String,
}
impl ErrorInfo {
    pub fn new(string: String, err: String) -> Self {
        Self { string, err }
    }
}
