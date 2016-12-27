#[cfg(test)]
use nom::IResult;

#[cfg(test)]
use std::fmt::Debug;


#[cfg(test)]
pub type Testcase<I: Sized, O: Sized> = (I, IResult<I, O>);

#[cfg(test)]
pub fn run_testcases<'a, T, Parser>(testcases: &Vec<Testcase<&'a[u8], T>>, test_parser: Parser)
where T: Eq,
      T: Debug,
      Parser: Fn(&'a [u8]) -> IResult<&'a[u8], T>
{

    let mut index = 0;
    for test in testcases {
        dbugln!("\nStarting test {}", index);
        let ref input = test.0;
        let ref expected_result = test.1;

        dbugln!("input : {:?}", String::from_utf8_lossy(input));
        dbugln!("expect: {:?}", &expected_result);

        let result = test_parser(input);
        dbugln!("actual: {:?}", result);

        assert_eq!(result, *expected_result);
        index += 1;
    }
}

