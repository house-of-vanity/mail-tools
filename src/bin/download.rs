extern crate chrono;
extern crate imap;
extern crate native_tls;

use mail_tools::*;

#[derive(Clone)]
pub struct Config {
    server: String,
    user: String,
    pass: String,
    port: u16,
    mailbox: String,
    dir: String,
}

fn main() {
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let config = Config {
        server: std::env::var("SERVER").unwrap(),
        user: std::env::var("USERNAME").unwrap(),
        pass: std::env::var("PASSWORD").unwrap(),
        port: std::env::var("PORT")
            .unwrap_or("993".into())
            .parse()
            .unwrap(),
        mailbox: std::env::var("MAILBOX").unwrap_or("INBOX".into()),
        dir: std::env::var("DIR").unwrap_or(format!(
            "{}/{}",
            std::env::var("USERNAME").unwrap(),
            std::env::var("MAILBOX").unwrap_or("INBOX".into())
        )),
    };

    let client = imap::connect(
        (config.server.clone(), config.port.clone()),
        config.server.clone(),
        &tls,
    )
    .unwrap();

    let mut imap_session = client
        .login(config.user.clone(), config.pass.clone())
        .unwrap();

    imap_session.select(config.mailbox.clone()).unwrap();
    std::fs::create_dir_all(config.dir.clone()).unwrap();
    messages_store_from(1, &mut imap_session, config.dir.clone());
}
