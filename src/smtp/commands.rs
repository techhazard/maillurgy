
#[derive(Debug, Eq, PartialEq)]
pub enum SmtpCommand {
    Hello(Greeting),
    EndOfTransmission,
    Message(String),
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
