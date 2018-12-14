# v0.1.2

- Changed logout function to use clear method on the sid.
- Changed logout in Drop to check using is_empty on the sid.
- Fixed wrong header value.

# v0.1.1

- Changed `&'static str` Client values to `String` due to lifetimes.
- Fixed incorrect accept_invalid_certs value in Client `Debug` formatter.

# v0.1.0

- First release
