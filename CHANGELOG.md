# v0.4.0

- Added call_and_check method
- Update query method to take a serde_json Value instead of a details-level
- Added query_and_check method

# v0.3.1

- Added `Client::read_only()` option to login.
- Added `Client::continue_last_session()` option to login.

# v0.3.0

- Removed error cause method as it was deprecated and replaced it with source.
- Improved error message if Response JSON is invalid.
- Update doc syntax error for `Client::accept_invalid_certs()`.

# v0.2.0

- Improved error handling for easier propagation.
- Added methods to save Response data and objects to a file.
- Removed Display for Response objects in favor of the save_objects method.
- Improved documentation to get started.

# v0.1.6

- Added quotes in a parse error to better show the problem value.
- Added "uid" to be stored in the Client.
- Clear api-server-version from the Client on successful logout.
- Added discard sessions example.
- Update existing examples.
- Update login_logout test.
- Updated doc.

# v0.1.5

- Update doc example.

# v0.1.4

- Added a `Custom` value for generic errors.
- Updated examples using this new value.
- Updated error while running a query.

# v0.1.3

- Changed missed if expressions to use is_empty instead of "".
- Added readme to Cargo.toml.
- Added tests.

# v0.1.2

- Changed logout function to use clear method on the sid.
- Changed logout in Drop to check using is_empty on the sid.
- Fixed wrong header value.

# v0.1.1

- Changed `&'static str` Client values to `String` due to lifetimes.
- Fixed incorrect accept_invalid_certs value in Client `Debug` formatter.

# v0.1.0

- First release
