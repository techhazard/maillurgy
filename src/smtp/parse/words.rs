
named!(pub crlf, tag!("\r\n"));
named!(pub space, eat_separator!(&b" \t"[..]));


named!(pub ehlo, tag!("EHLO "));
named!(pub helo, tag!("HELO "));
named!(pub ehelo, alt!(ehlo|helo));

named!(pub quit, tag!("QUIT"));

#[cfg(test)]
mod tests {
    use nom::IResult::*; // Done, Error, Incomplete
    use nom::Needed;
    use nom::ErrorKind::*; // Tag, Complete, Alt ... see http://rust.unhandledexpression.com/nom/enum.ErrorKind.html
    use nom::Err::Position;
    use super::super::testhelper::*;

    #[test]
    fn quit_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], &[u8]>> = vec![
            (b"QUIT", Done(b"", b"QUIT")),
            (b"QUIT\r\n", Done(b"\r\n", b"QUIT")),
            (b"Q", Incomplete(Needed::Size(4))),
            (b"QU", Incomplete(Needed::Size(4))),
            (b"QUI", Incomplete(Needed::Size(4))),
            (b"  QUIT", Error(Position(Tag, b"  QUIT"))),
        ];
        run_testcases(&testcases, super::quit);

    }
    #[test]
    fn crlf_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8],&[u8]>> = vec![
            (b"\r\n", Done(b"", b"\r\n")),
            (b"\r\naa", Done(b"aa", b"\r\n")),
        ];
        run_testcases(&testcases, super::crlf);
    }
    #[test]
    fn ehlo_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], &[u8]>> = vec![
            (b"EHLO ", Done(b"", b"EHLO ")),
            (b"EHLO domain.com", Done(b"domain.com", b"EHLO ")),
        ];
        run_testcases(&testcases, super::ehlo);
    }
    #[test]
    fn helo_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], &[u8]>> = vec![
            (b"HELO ", Done(b"", b"HELO ")),
            (b"HELO domain.com", Done(b"domain.com", b"HELO ")),
        ];
        run_testcases(&testcases, super::helo);
    }
    #[test]
    fn ehelo_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], &[u8]>> = vec![
            (b"HELO ", Done(b"", b"HELO ")),
            (b"EHLO ", Done(b"", b"EHLO ")),
            (b"HELO domain.com", Done(b"domain.com", b"HELO ")),
            (b"EHLO domain.com", Done(b"domain.com", b"EHLO ")),
        ];
        run_testcases(&testcases, super::ehelo);
    }
}
