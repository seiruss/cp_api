use cp_api::Client;

#[test]
fn show_hosts() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.login("cp_api", "vpn123").unwrap();

    println!("Querying all hosts...");

    let hosts = client.query("show-hosts", "standard").unwrap();

    for host in hosts.objects {
        println!("{} - {}", host["name"], host["ipv4-address"]);
    }
}
