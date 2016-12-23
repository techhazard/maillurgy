use std::str::from_utf8;

named!(crlf, tag!("\r\n"));
named!(space, eat_separator!(&b" \t"[..]));


named!(ehlo, tag!("EHLO "));
named!(helo, tag!("HELO "));
named!(ehelo, alt!(ehlo|helo));

named!(quit, tag!("QUIT"));
named!(pub end_of_transmission, complete!(do_parse!(
    quit >>
    crlf >>
    (b"")
  ))
);
// named!(end_of_transmission, do_parse!(
        // quit >>
        // crlf
        // ));


named!(pub greeting<(&str, &str)>, do_parse!(
    hello: ehelo >>
    identifier: take_until!("\r\n") >>
    line_end: crlf >>
    (from_utf8(hello).unwrap(), from_utf8(identifier).unwrap())
  )
);

use nom::be_u8;
named!(tag_length_value<(u8, &[u8])>,
  do_parse!(
    tag!( &[ 42u8 ][..] ) >>
    length: be_u8         >>
    bytes:  take!(length) >>
    (length, bytes)
  )
);


// named!(greeting, do_parse!(
        // ehelo: ehelo >>
        // remainder: take_till!(crlf)
    // )
// );


#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::{self,Done,Error,Incomplete};
    use nom::{Needed,ErrorKind, Err as NomErr};
    use std::str::from_utf8;
    use std::fmt::Debug;

    //struct Testcase<I,O>(I, IResult<I, O>) where O: Sized, I: Sized;
    type Testcase<I: Sized, O: Sized> = (I, IResult<I, O>);

    fn run_testcases<'a, T, Parser>(testcases: &Vec<Testcase<&'a[u8], T>>, test_parser: Parser)
        where T: Eq,
              T: Debug,
              Parser: Fn(&'a [u8]) -> IResult<&'a[u8], T>
    {
        println!("");

        for test in testcases {
            let ref input = test.0;
            let ref expected_result = test.1;

            println!("input : {:?}", String::from_utf8_lossy(input));

            println!("expect: {:?}", &expected_result);

            let result = test_parser(input);
            println!("actual: {:?}", result);

            assert_eq!(result, *expected_result);
        }
    }

    #[test]
    fn quit_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], &[u8]>> = vec![
            (b"QUIT", Done(b"", b"QUIT")),
            (b"QUIT\r\n", Done(b"\r\n", b"QUIT")),
            (b"Q", Incomplete(Needed::Size(4))),
            (b"QU", Incomplete(Needed::Size(4))),
            (b"QUI", Incomplete(Needed::Size(4))),
            (b"  QUIT", Error(NomErr::Position(ErrorKind::Tag, b"  QUIT"))),
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
    #[test]
    fn greeting_test() {
        // input, exp parsed, exp unparsed
        let testcases : Vec<Testcase<&[u8], (&str, &str)>> = vec![
            (b"EHLO domain.com\r\n\
               MAIL FROM: <me@domain.com>\r\n",
             Done(b"MAIL FROM: <me@domain.com>\r\n", ("EHLO ","domain.com") )),

            (b"HELO domain.com\r\nMAIL FROM: <me@domain.com>\r\n",
             Done(b"MAIL FROM: <me@domain.com>\r\n", ("HELO ","domain.com") )),
        ];
            run_testcases(&testcases, super::greeting);
    }
}



// Server Response: 220 www.sample.com ESMTP Postfix
// Client Sending : HELO domain.com
// Server Response: 250 Hello domain.com
// Client Sending : MAIL FROM: <me@domain.com>
// Server Response: 250 Ok
// Client Sending : RCPT TO: <friend@sample.com>
// Server Response: 250 Ok
// Client Sending : DATA
// Server Response: 354 End data with <CR><LF>.<CR><LF>
// Client Sending : Subject: Example Message
// Client Sending : From: me@domain.com
// Client Sending : To: you@sample.com
// Client Sending :
// Client Sending : Yo,
// Client Sending :
// Client Sending :   Sending a test message.
// Client Sending :
// Client Sending :   Later,
// Client Sending : Carl
// Client Sending : .
// Server Response: 250 Ok: queued as 45334
// Client Sending : QUIT
// Server Response: 221 Bye
