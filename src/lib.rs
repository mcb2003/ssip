mod command;
pub use command::Command;
mod speech;
pub use speech::Speech;

use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum ClientId {
    All,
    Self_,
    Id(i32),
}

impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Self_ => write!(f, "self"),
            Self::Id(id) => write!(f, "{}", id),
        }
    }
}
