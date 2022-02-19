use std::fmt;

use super::{CapitalsMode, NotificationParam, Priority, PunctuationLevel};

#[derive(Clone, Debug)]
pub enum SetParam<'a> {
    Priority(Priority),
    ClientName {
        user_name: &'a str,
        client_name: &'a str,
        connection_name: &'a str,
    },
    Debug(bool),
    OutputModule(&'a str),
    Language(&'a str),
    SsmlMode(bool),
    Punctuation(PunctuationLevel),
    Spelling(bool),
    Capitals(CapitalsMode),
    VoiceType(&'a str),
    SynthesisVoice(&'a str),
    Rate(i8),
    Pitch(i8),
    Volume(i8),
    PauseContext(usize),
    History(bool),
    Notification(NotificationParam, bool),
}

impl fmt::Display for SetParam<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Priority(p) => write!(f, "priority {}", p),
            Self::ClientName {
                user_name,
                client_name,
                connection_name,
            } => write!(
                f,
                "client_name {}:{}:{}",
                user_name, client_name, connection_name
            ),
            Self::Debug(switch) => write!(f, "debug {}", if *switch { "on" } else { "off" }),
            Self::OutputModule(m) => write!(f, "output_module {}", m),
            Self::Language(code) => write!(f, "language {}", code),
            Self::SsmlMode(mode) => write!(f, "ssml_mode {}", if *mode { "on" } else { "off" }),
            Self::Punctuation(p) => write!(f, "punctuation {}", p),
            Self::Spelling(mode) => write!(f, "spelling {}", if *mode { "on" } else { "off" }),
            Self::Capitals(c) => write!(f, "cap_let_recogn {}", c),
            Self::VoiceType(t) => write!(f, "voice_type {}", t),
            Self::SynthesisVoice(v) => write!(f, "synthesis_voice {}", v),
            Self::Rate(r) => write!(f, "rate {}", r),
            Self::Pitch(p) => write!(f, "pitch {}", p),
            Self::Volume(v) => write!(f, "volume {}", v),
            Self::PauseContext(c) => write!(f, "pause_context {}", c),
            Self::History(switch) => write!(f, "history {}", if *switch { "on" } else { "off" }),
            Self::Notification(param, switch) => write!(
                f,
                "notification {} {}",
                param,
                if *switch { "on" } else { "off" }
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        let p = SetParam::Priority(Priority::Important);
        assert_eq!(p.to_string(), "priority important");
    }

    #[test]
    fn test_client_name() {
        let p = SetParam::ClientName {
            user_name: "user",
            client_name: "client",
            connection_name: "main",
        };
        assert_eq!(p.to_string(), "client_name user:client:main");
    }

    #[test]
    fn test_debug() {
        let p = SetParam::Debug(true);
        assert_eq!(p.to_string(), "debug on");
        let p = SetParam::Debug(false);
        assert_eq!(p.to_string(), "debug off");
    }

    #[test]
    fn test_output_module() {
        let p = SetParam::OutputModule("espeak-ng");
        assert_eq!(p.to_string(), "output_module espeak-ng");
    }

    #[test]
    fn test_language() {
        let p = SetParam::Language("en-gb");
        assert_eq!(p.to_string(), "language en-gb");
    }

    #[test]
    fn test_ssml_mode() {
        let p = SetParam::SsmlMode(true);
        assert_eq!(p.to_string(), "ssml_mode on");
        let p = SetParam::SsmlMode(false);
        assert_eq!(p.to_string(), "ssml_mode off");
    }
    #[test]
    fn test_punctuation() {
        let p = SetParam::Punctuation(PunctuationLevel::Most);
        assert_eq!(p.to_string(), "punctuation most");
    }

    #[test]
    fn test_spelling() {
        let p = SetParam::Spelling(true);
        assert_eq!(p.to_string(), "spelling on");
        let p = SetParam::SsmlMode(false);
        assert_eq!(p.to_string(), "ssml_mode off");
    }

    #[test]
    fn test_capitals() {
        let p = SetParam::Capitals(CapitalsMode::Icon);
        assert_eq!(p.to_string(), "cap_let_recogn icon");
    }

    #[test]
    fn test_voice_type() {
        let p = SetParam::VoiceType("male1");
        assert_eq!(p.to_string(), "voice_type male1");
    }

    #[test]
    fn test_synthesis_voice() {
        let p = SetParam::SynthesisVoice("English (Great Britain)+Max");
        assert_eq!(p.to_string(), "synthesis_voice English (Great Britain)+Max");
    }

    #[test]
    fn test_rate() {
        let p = SetParam::Rate(-100);
        assert_eq!(p.to_string(), "rate -100");
        let p = SetParam::Rate(100);
        assert_eq!(p.to_string(), "rate 100");
    }

    #[test]
    fn test_pitch() {
        let p = SetParam::Pitch(-100);
        assert_eq!(p.to_string(), "pitch -100");
        let p = SetParam::Pitch(100);
        assert_eq!(p.to_string(), "pitch 100");
    }

    #[test]
    fn test_volume() {
        let p = SetParam::Volume(-100);
        assert_eq!(p.to_string(), "volume -100");
        let p = SetParam::Volume(100);
        assert_eq!(p.to_string(), "volume 100");
    }

    #[test]
    fn test_pause_context() {
        let p = SetParam::PauseContext(2);
        assert_eq!(p.to_string(), "pause_context 2");
    }

    #[test]
    fn test_history() {
        let p = SetParam::History(true);
        assert_eq!(p.to_string(), "history on");
        let p = SetParam::History(false);
        assert_eq!(p.to_string(), "history off");
    }

    #[test]
    fn test_notification() {
        let p = SetParam::Notification(NotificationParam::All, true);
        assert_eq!(p.to_string(), "notification all on");
        let p = SetParam::Notification(NotificationParam::All, false);
        assert_eq!(p.to_string(), "notification all off");
    }
}
