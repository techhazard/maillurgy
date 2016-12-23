use super::message::Message;

pub enum SmtpCommand {
    Greeting(Greeting),
    EndOfTransmission,
    Message(Message),

}

pub enum Greeting {
    EHLO,
    HELO
}
