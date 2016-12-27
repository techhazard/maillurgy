#[macro_use]
extern crate nom;

#[macro_export]
macro_rules! dbug {
    ($($arg:tt)*) => (#[cfg(any(test, feature = "debug"))] print!($($arg)*));
}

#[macro_export]
macro_rules! dbugln {
    ($($arg:tt)*) => (#[cfg(any(test, feature = "debug"))] println!($($arg)*));
}

pub mod socket;
pub mod smtp;


#[test]
fn maco_test() {
    dbug!("debug!!!!!! {}\n", 4);
    dbug!("debug!!!!!! {:?}\n", 5);
    dbugln!("debug!!!!!! {}\n", 6);
    dbugln!("debug!!!!!! {:?}\n", 7);
}
