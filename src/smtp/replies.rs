#[derive(Eq, PartialEq)]
pub enum Reply {
    SyntaxError,
    UnrecognisedCommand,
    CommandNotImplemented,
    BadSequence,
    ParameterNotImplemented,
}

impl Reply {
    fn code(&self) -> &[u8] {
        use self::Reply::*;
        match *self {
            SyntaxError => b"500 Syntax error, command unrecognized",
            UnrecognisedCommand => b"501 Syntax error in parameters or arguments",
            CommandNotImplemented => b"502 Command not implemented",
            BadSequence => b"503 Bad sequence of commands",
            ParameterNotImplemented => b"504 Command parameter not implemented",
        }
    }
}


#[cfg(test)]
mod test {
    use super::Reply::*;

    #[test]
    fn test_replies() {
        // slice needed due to type comparison
        assert_eq!(SyntaxError.code(), &b"500 Syntax error, command unrecognized"[..]);
        assert_eq!(UnrecognisedCommand.code(), &b"501 Syntax error in parameters or arguments"[..]);
        assert_eq!(CommandNotImplemented.code(), &b"502 Command not implemented"[..]);
        assert_eq!(BadSequence.code(), &b"503 Bad sequence of commands"[..]);
        assert_eq!(ParameterNotImplemented.code(), &b"504 Command parameter not implemented"[..]);
    }
}
//    x1z  Information: These are replies to requests for information, such
//         as status or help.
//
//    x2z  Connections: These are replies referring to the transmission
//         channel.
//
//    x3z  Unspecified.
//
//    x4z  Unspecified.
//
//    x5z  Mail system: These replies indicate the status of the receiver
//         mail system vis-a-vis the requested transfer or other mail system
//         action.
