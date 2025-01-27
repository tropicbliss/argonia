# argonian

A lightweight password hashing library using Argon2, built with WebAssembly for environments without Node.js. Perfect for serverless platforms like Cloudflare Workers. This implementation matches [@node-rs/argon2](https://github.com/napi-rs/node-rs/tree/main/packages/argon2)'s output exactly.

> **Important:** This is built specifically for WebAssembly environments like Cloudflare Workers. For Node.js applications, use @node-rs/argon2 instead.

## Features

- Secure password hashing using Argon2
- WebAssembly-based implementation
- Zero dependencies
- Consistent behavior across platforms
- Simple, straightforward API

## Installation

```bash
npm install argonian
```

## Usage

```js
import { hash, verify } from 'argonian';

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

## Security

This package uses Argon2, the winner of the Password Hashing Competition. The implementation uses secure default parameters that are suitable for most use cases.

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
- [RustCrypto](https://github.com/RustCrypto/password-hashes/tree/master/argon2) - The Rust implementation of Argon2
- [@node-rs/argon2](https://github.com/napi-rs/node-rs/tree/main/packages/argon2) - The reference implementation
