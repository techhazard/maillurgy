// TODO: finish
/// actual email and its data

#[derive(Debug, Eq, PartialEq)]
pub struct Message {
    headers: Vec<String>,
    to_address: Vec<String>,
    cc_address: Vec<String>,
    bcc_address: Vec<String>,
    body: Vec<String>
    /* more? */
}
