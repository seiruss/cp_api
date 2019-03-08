use cp_api::Client;
use std::error::Error;

#[test]
fn desc_source() {
    let mut client = Client::new("192.168.1.12", 443);
    client.accept_invalid_certs(true);

    if let Err(e) = client.login("cp_api", "abc123") {
        eprintln!("error: {}", e);
        eprintln!("description: {}", e.description());
        if e.source.is_some() {
            eprintln!("source: {}", e.source().unwrap());
        }
        else {
            eprintln!("no source");
        }
    }
}

#[test]
fn io_error() {
    let mut client = Client::new("172.25.199.80", 443);
    client.certificate("cert.cer");
}
