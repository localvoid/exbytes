# exbytes

Extensions for [bytes](https://crates.io/crates/bytes) providing variable-length integer encoding via LEB128 (unsigned) and ZigZag (signed).

## Usage

```rust
use bytes::BytesMut;
use exbytes::{BytesMutExt as _, BytesExt as _};

let mut buf = BytesMut::new();
buf.put_vu32(42u32);
buf.put_zz32(-7i32);

let mut view = buf.freeze();
let a = view.get_vu32();   // 42
let b = view.get_zz32();   // -7
```

Fallible decoding is also available via `try_get_*` methods:

```rust
let mut buf: &[u8] = &[0x80, 0x01];
let n = buf.try_get_vu16()?;  // 128
```

## Supported types

| Encoding | `get` / `put`                   | `try_get`                          |
| -------- | ------------------------------- | ---------------------------------- |
| LEB128   | `vu16`, `vu32`, `vu64`, `vu128` | `try_get_vu16`, …, `try_get_vu128` |
| ZigZag   | `zz16`, `zz32`, `zz64`, `zz128` | `try_get_zz16`, …, `try_get_zz128` |

## License

MIT or Apache-2.0
