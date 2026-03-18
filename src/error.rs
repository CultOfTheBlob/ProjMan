#[derive(Debug, Clone)]
pub struct ErrorInfo
{
    pub string: String,
    pub err: String,
}

#[derive(Debug, Clone)]
pub enum Error
{
    Other(String),
    Create(ErrorInfo),
    WriteTo(ErrorInfo),
    Find(ErrorInfo),
    Remove(ErrorInfo),
    Read(ErrorInfo),
    Parse(ErrorInfo),
    Clone(ErrorInfo),
    Fetch(ErrorInfo),
    Run(ErrorInfo),
    Commit(ErrorInfo),
}

impl Error
{
    pub fn get_message(&self) -> String
    {
        match self
        {
            Error::Other(message) => format!("Error: {message}"),
            Error::Create(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not create {string} ({err})")
            }
            Error::WriteTo(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not write to {string} ({err})")
            }
            Error::Find(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not find {string} ({err})")
            }
            Error::Remove(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not remove {string} file ({err})")
            }
            Error::Read(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not read {string} ({err})")
            }
            Error::Parse(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not parse {string} ({err})")
            }
            Error::Clone(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not clone {string} ({err})")
            }
            Error::Fetch(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not fetch {string} ({err})")
            }
            Error::Run(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not run [{string}] ({err})")
            }
            Error::Commit(ErrorInfo { string, err }) =>
            {
                format!("Error: Could not commit {string} ({err})")
            }
        }
    }
}
