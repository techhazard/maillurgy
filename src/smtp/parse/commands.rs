use std::str::from_utf8;

use super::words::*;

named!(pub end_of_transmission, complete!(do_parse!(
    quit >>
    crlf >>
    (b"")
  ))
);

named!(pub greeting<(&str, &str)>, do_parse!(
    hello: ehelo >>
    identifier: take_until!("\r\n") >>
    line_end: crlf >>
    (from_utf8(hello).unwrap(), from_utf8(identifier).unwrap())
  )
);


#[cfg(test)]
mod tests {
    use nom::Needed;
    use nom::IResult::*; // Done, Error, Incomplete
    use nom::ErrorKind::*; // Tag, Complete, Alt ... see http://rust.unhandledexpression.com/nom/enum.ErrorKind.html
    use nom::Err::Position;
    use smtp::parse::testhelper::*; // Testcase<I, O>, run_testcases()

    #[test]
    fn end_of_transmission_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], &[u8]>> = vec![
            (b"QUIT", Error(Position(Complete, b"QUIT"))),
            (b"QUIT\r\n", Done(b"", b"")),
            (b"Q", Error(Position(Complete, b"Q"))),
            (b"QU", Error(Position(Complete, b"QU"))),
            (b"QUI", Error(Position(Complete, b"QUI"))),
        ];
        run_testcases(&testcases, super::end_of_transmission);

    }
    #[test]
    fn greeting_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], (&str, &str)>> = vec![
            (b"EHLO domain.com\r\n\
               MAIL FROM: <me@domain.com>\r\n",
             Done(b"MAIL FROM: <me@domain.com>\r\n", ("EHLO ","domain.com") )),

            (b"HELO domain.com\r\nMAIL FROM: <me@domain.com>\r\n",
             Done(b"MAIL FROM: <me@domain.com>\r\n", ("HELO ","domain.com") )),

            (b"HELO domain",
             Error(Position(TakeUntil, b"domain"))),
        ];
            run_testcases(&testcases, super::greeting);
    }
}
