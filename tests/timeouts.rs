use cp_api::Client;

#[test]
fn connect_timeout() {
    let mut client = Client::new("1.1.1.1", 443);
    client.accept_invalid_certs(true);
    client.connect_timeout(5);
    client.login("cp_api", "vpn123").unwrap();
    client.logout().unwrap();
}

// If below test name does not have "test" in it, the compiler sees this
// function comment as a test. This does not seem to occur with the above test.
/*
running 1 test
error[E0425]: cannot find value `client` in this scope
 --> src\client.rs:340:1
  |
3 | client.session_timeout(1200);
  | ^^^^^^ not found in this scope

thread 'src\client.rs - client::Client::session_timeout (line 339)' panicked at 'couldn't compile the test', librustdoc\test.rs:332:13
*/
#[test]
fn session_timeout_test() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.session_timeout(800);
    let res = client.login("cp_api", "vpn123").unwrap();
    assert_eq!(800, res.data["session-timeout"]);
    client.logout().unwrap();
}
