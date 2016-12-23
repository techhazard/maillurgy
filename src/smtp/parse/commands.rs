use std::str::from_utf8;

use super::words::*;
use super::super::commands::SmtpCommand;
use super::super::commands;

named!(pub end_of_transmission<SmtpCommand>, complete!(do_parse!(
    quit >>
    crlf >>
    (SmtpCommand::EndOfTransmission)
  ))
);

named!(pub greeting<SmtpCommand>, do_parse!(
    hello: ehelo >>
    identifier: take_until!("\r\n") >>
    line_end: crlf >>
    (parse_greeting(hello,identifier))
  )
);

fn parse_greeting(hello: &[u8], identifier: &[u8]) -> SmtpCommand {
    let hello_str = from_utf8(hello).unwrap();
    let identifier = from_utf8(identifier);
    if identifier.is_err() {
        return SmtpCommand::InvalidCommand
    }
    let hello = match hello_str {
        "HELO " => commands::GreetingType::HELO,
        "EHLO " => commands::GreetingType::EHLO,

        _ => return SmtpCommand::InvalidCommand,
    };
    let greeting = commands::Greeting{
        hello: hello,
        identifier: identifier.unwrap().to_string()
    };
    SmtpCommand::Hello(greeting)
}

#[cfg(test)]
mod tests {
    use nom::Needed;
    use nom::IResult::*; // Done, Error, Incomplete
    use nom::ErrorKind::*; // Tag, Complete, Alt ... see http://rust.unhandledexpression.com/nom/enum.ErrorKind.html
    use nom::Err::Position;
    use smtp::parse::testhelper::*; // Testcase<I, O>, run_testcases()
    use smtp::commands::SmtpCommand::{self, Hello, InvalidCommand};
    use smtp::commands::{Greeting, GreetingType};

    #[test]
    fn end_of_transmission_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], SmtpCommand>> = vec![
            (b"QUIT", Error(Position(Complete, b"QUIT"))),
            (b"QUIT\r\n", Done(b"", SmtpCommand::EndOfTransmission)),
            (b"Q", Error(Position(Complete, b"Q"))),
            (b"QU", Error(Position(Complete, b"QU"))),
            (b"QUI", Error(Position(Complete, b"QUI"))),
        ];
        run_testcases(&testcases, super::end_of_transmission);

    }
    #[test]
    fn greeting_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], SmtpCommand>> = vec![
            (b"EHLO domain.com\r\n\
               MAIL FROM: <me@domain.com>\r\n",
             Done(b"MAIL FROM: <me@domain.com>\r\n",
                  Hello(Greeting{hello: GreetingType::EHLO,identifier: "domain.com".to_string()}))),

            (b"HELO domain.com\r\n\
               MAIL FROM: <me@domain.com>\r\n",
             Done(b"MAIL FROM: <me@domain.com>\r\n",
                  Hello(Greeting{hello: GreetingType::HELO,identifier: "domain.com".to_string()}))),

            (b"HELO domain",
             Error(Position(TakeUntil, b"domain"))),
        ];
            run_testcases(&testcases, super::greeting);
    }
}
