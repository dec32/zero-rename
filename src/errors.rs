use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error)
}

pub type Result<T> = core::result::Result<T, Error>;


impl Error {
    fn to_string(&self) -> String {
        match self {
            Self::IO(err) => format!("{}", err)
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&self.to_string())
    }
}