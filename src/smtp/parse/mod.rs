mod testhelper;
mod words;
pub mod commands;





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
