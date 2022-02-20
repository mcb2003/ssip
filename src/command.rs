use std::fmt;

use super::{ClientId, GetParam, SetParam, Speech};

#[derive(Clone, Debug)]
pub enum Command<'a> {
    Speak(Speech<'a>),
    Char(char),
    SoundIcon(&'a str),
    Stop(ClientId),
    Cancel(ClientId),
    Pause(ClientId),
    Resume(ClientId),
    Set {
        client: ClientId,
        param: SetParam<'a>,
    },
    Get(GetParam),
}

impl fmt::Display for Command<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Speak(text) => write!(f, "speak\r\n{}\r\n.\r\n", text),
            Self::Char(c) if *c == ' ' => write!(f, "char space\r\n"),
            Self::Char(c) => write!(f, "char {}\r\n", c),
            Self::SoundIcon(s) => write!(f, "sound_icon {}\r\n", s),
            Self::Stop(id) => write!(f, "stop {}\r\n", id),
            Self::Cancel(id) => write!(f, "cancel {}\r\n", id),
            Self::Pause(id) => write!(f, "pause {}\r\n", id),
            Self::Resume(id) => write!(f, "resume {}\r\n", id),
            Self::Set { client, param } => write!(f, "set {} {}\r\n", client, param),
            Self::Get(param) => write!(f, "get {}\r\n", param),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Priority;

    #[test]
    fn test_speak_display() {
        let cmd = Command::Speak(Speech::from("Hello, world!"));
        assert_eq!(cmd.to_string(), "speak\r\nHello, world!\r\n.\r\n");
    }

    #[test]
    fn test_char_display() {
        let cmd = Command::Char('A');
        assert_eq!(cmd.to_string(), "char A\r\n");
        let cmd = Command::Char(' ');
        assert_eq!(cmd.to_string(), "char space\r\n");
    }

    #[test]
    fn test_sound_icon_display() {
        let cmd = Command::SoundIcon("beep");
        assert_eq!(cmd.to_string(), "sound_icon beep\r\n");
    }

    #[test]
    fn test_stop_display() {
        let cmd = Command::Stop(ClientId::All);
        assert_eq!(cmd.to_string(), "stop all\r\n");
        let cmd = Command::Stop(ClientId::Self_);
        assert_eq!(cmd.to_string(), "stop self\r\n");
        let cmd = Command::Stop(ClientId::Id(123));
        assert_eq!(cmd.to_string(), "stop 123\r\n");
    }

    #[test]
    fn test_cancel_display() {
        let cmd = Command::Cancel(ClientId::All);
        assert_eq!(cmd.to_string(), "cancel all\r\n");
        let cmd = Command::Cancel(ClientId::Self_);
        assert_eq!(cmd.to_string(), "cancel self\r\n");
        let cmd = Command::Cancel(ClientId::Id(123));
        assert_eq!(cmd.to_string(), "cancel 123\r\n");
    }

    #[test]
    fn test_pause_display() {
        let cmd = Command::Pause(ClientId::All);
        assert_eq!(cmd.to_string(), "pause all\r\n");
        let cmd = Command::Pause(ClientId::Self_);
        assert_eq!(cmd.to_string(), "pause self\r\n");
        let cmd = Command::Pause(ClientId::Id(123));
        assert_eq!(cmd.to_string(), "pause 123\r\n");
    }

    #[test]
    fn test_resume_display() {
        let cmd = Command::Resume(ClientId::All);
        assert_eq!(cmd.to_string(), "resume all\r\n");
        let cmd = Command::Resume(ClientId::Self_);
        assert_eq!(cmd.to_string(), "resume self\r\n");
        let cmd = Command::Resume(ClientId::Id(123));
        assert_eq!(cmd.to_string(), "resume 123\r\n");
    }

    #[test]
    fn test_set() {
        // We don't test all possibilities here, since they're done in the tests for Param
        let cmd = Command::Set {
            client: ClientId::Self_,
            param: SetParam::Priority(Priority::Message),
        };
        assert_eq!(cmd.to_string(), "set self priority message\r\n");
    }

    #[test]
    fn test_get() {
        let cmd = Command::Get(GetParam::OutputModule);
        assert_eq!(cmd.to_string(), "get output_module\r\n");
    }
}
