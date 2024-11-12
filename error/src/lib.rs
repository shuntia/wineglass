use std::fmt::{self, Display, Formatter};

pub struct ErrorLogger {
    errors: Vec<Box<dyn Error>>,
}

trait Error: std::fmt::Display {
    fn get_position(&self) -> Position;
    fn get_errorlevel(&self) -> ErrorLevel;
}
#[derive(Debug, Clone)]
pub enum ErrorLevel {
    INFO,
    WARN,
    ERROR,
    FATAL,
}
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

pub struct Parse {
    pub errorlevel: ErrorLevel,
    pub message: String,
    pub position: Position,
}

pub struct Runtime {
    pub errorlevel: ErrorLevel,
    pub message: String,
    pub position: Position,
}

pub struct Syntax {
    pub errorlevel: ErrorLevel,
    pub message: String,
    pub position: Position,
}

impl Error for Parse {
    fn get_position(&self) -> Position {
        self.position.clone()
    }

    fn get_errorlevel(&self) -> ErrorLevel {
        self.errorlevel.clone()
    }
}

impl Display for Parse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.message, self.position.line, self.position.column
        )
    }
}

impl Error for Runtime {
    fn get_position(&self) -> Position {
        self.position.clone()
    }

    fn get_errorlevel(&self) -> ErrorLevel {
        self.errorlevel.clone()
    }
}

impl Display for Runtime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.message, self.position.line, self.position.column
        )
    }
}

impl Error for Syntax {
    fn get_position(&self) -> Position {
        self.position.clone()
    }

    fn get_errorlevel(&self) -> ErrorLevel {
        self.errorlevel.clone()
    }
}

impl Display for Syntax {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.message, self.position.line, self.position.column
        )
    }
}

macro_rules! parse_error {
    ($position:expr, $message:expr) => {
        Parse {
            errorlevel: ErrorLevel::ERROR,
            message: $message.to_string(),
            position: $position,
        }
    };
}

macro_rules! parse_warning {
    ($position:expr, $message:expr) => {
        Parse {
            errorlevel: ErrorLevel::WARN,
            message: $message.to_string(),
            position: $position,
        }
    };
}

macro_rules! parse_info {
    ($position:expr, $message:expr) => {
        Parse {
            errorlevel: ErrorLevel::INFO,
            message: $message.to_string(),
            position: $position,
        }
    };
}

macro_rules! parse_fatal {
    ($position:expr, $message:expr) => {
        Parse {
            errorlevel: ErrorLevel::FATAL,
            message: $message.to_string(),
            position: $position,
        }
    };
}
