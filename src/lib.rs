mod command;
pub use command::Command;
mod set_param;
pub use set_param::SetParam;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GetParam {
    OutputModule,
    VoiceType,
    Rate,
    Pitch,
    Volume,
}

impl fmt::Display for GetParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutputModule => write!(f, "output_module"),
            Self::VoiceType => write!(f, "voice_type"),
            Self::Rate => write!(f, "rate"),
            Self::Pitch => write!(f, "pitch"),
            Self::Volume => write!(f, "volume"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListParam {
    OutputModules,
    Voices,
    SynthesisVoices,
}

impl fmt::Display for ListParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutputModules => write!(f, "output_modules"),
            Self::Voices => write!(f, "voices"),
            Self::SynthesisVoices => write!(f, "synthesis_voices"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationParam {
    All,
    Begin,
    End,
    Pause,
    Resume,
    IndexMark,
}

impl fmt::Display for NotificationParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Begin => write!(f, "begin"),
            Self::End => write!(f, "end"),
            Self::Pause => write!(f, "pause"),
            Self::Resume => write!(f, "resume"),
            Self::IndexMark => write!(f, "index_mark"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Progress,
    Notification,
    Text,
    Message,
    Important,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Progress => write!(f, "progress"),
            Self::Notification => write!(f, "notification"),
            Self::Text => write!(f, "text"),
            Self::Message => write!(f, "message"),
            Self::Important => write!(f, "important"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PunctuationLevel {
    None,
    Some,
    Most,
    All,
}

impl fmt::Display for PunctuationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Some => write!(f, "some"),
            Self::Most => write!(f, "most"),
            Self::All => write!(f, "all"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapitalsMode {
    None,
    Icon,
    Spell,
}

impl fmt::Display for CapitalsMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Icon => write!(f, "icon"),
            Self::Spell => write!(f, "spell"),
        }
    }
}
