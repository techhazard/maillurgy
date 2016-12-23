use std::fmt;
use std::fmt::Debug;

use super::message::Message;

#[derive(Debug, Eq, PartialEq)]
pub enum SmtpCommand {
    Hello(Greeting),
    EndOfTransmission,
    Message(Message),
    InvalidCommand,
}


#[derive(Debug, Eq, PartialEq)]
pub enum GreetingType {
    EHLO,
    HELO
}

#[derive(Debug, Eq, PartialEq)]
pub struct Greeting {
    pub hello : GreetingType,
    pub identifier: String
}
