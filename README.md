# @tropicbliss/argonian

A simple and opinionated password hashing library using Argon2. Powered by WebAssembly and designed to be used in environments that do not support the Node-API. This package guarantees identical behavior with @node-rs/argon2.

## Features

- Secure password hashing using Argon2
- WebAssembly-based implementation
- Zero dependencies
- Consistent behavior across platforms
- Simple, straightforward API

## Installation

```bash
npm install @tropicbliss/argonian
```

## Usage

```javascript
import { hash, verify } from '@tropicbliss/argonian';

const password = "argonian";

try {
  // Hash a password
  const hashed = hash(password);
  
  // Verify a password against a hash
  const isValid = verify(hashed, password);
  console.log('Password verification:', isValid); // true
} catch (e) {
  console.error("Failed to hash and verify password:", e);
}
```

## API

### hash(password: string): string

Hashes a password using Argon2 with secure default parameters.

**Parameters:**
- `password`: The password string to hash

**Returns:**
- A string containing the hashed password

**Throws:**
- `Error` if hashing fails

### verify(hash: string, password: string): boolean

Verifies a password against a hash.

**Parameters:**
- `hash`: The hashed password string
- `password`: The password to verify

**Returns:**
- `true` if the password matches the hash
- `false` if the password does not match

**Throws:**
- `Error` if verification fails due to invalid hash format

## Security

This package uses Argon2, the winner of the Password Hashing Competition. The implementation uses secure default parameters that are suitable for most use cases.

## Browser Support

This package works in all modern browsers that support WebAssembly:
- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- Opera 44+

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in argonian by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

- [wasm-pack](https://github.com/rustwasm/wasm-pack) - The WebAssembly build tool that made this package possible
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - For making Rust and WebAssembly work together seamlessly
- [rust-argon2](https://github.com/sru-systems/rust-argon2) - The Rust implementation of Argon2
- [@node-rs/argon2](https://github.com/napi-rs/node-rs/tree/main/packages/argon2) - The reference implementation
