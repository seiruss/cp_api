use cp_api::Client;

#[test]
fn get_headers() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);

    let login = client.login("cp_api", "vpn123").unwrap();
    println!("{:#?}", login.headers());

    client.logout().unwrap();
}

#[test]
fn display_objects() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.login("cp_api", "vpn123").unwrap();

    let hosts = client.query("show-hosts", "standard").unwrap();
    println!("{:#}", hosts);

    client.logout().unwrap();
}

#[test]
fn print_response() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    let login = client.login("cp_api", "vpn123").unwrap();

    println!("{:#?}", login);

    client.logout().unwrap();
}

#[test]
fn print_client() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.certificate("cert.cer");
    client.proxy("https://192.168.1.12:8080");
    client.domain("test domain");
    client.log_file("test_log.txt");

    client.login("cp_api", "vpn123").unwrap();
    println!("{:#?}", client);

    client.logout().unwrap();
}
