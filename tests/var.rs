use bytes::{Buf, Bytes, BytesMut};
use exbytes::{BytesExt as _, BytesMutExt as _};
use std::fmt::Debug;

fn roundtrip_unsigned<T: Eq + Debug + Copy>(
    encode: impl FnOnce(&mut BytesMut, T),
    decode: impl FnOnce(&mut Bytes) -> T,
    val: T,
) {
    let mut w = BytesMut::new();
    encode(&mut w, val);
    let mut b = w.freeze();
    assert_eq!(decode(&mut b), val);
    assert_eq!(b.remaining(), 0);
}

fn roundtrip_signed<T: Eq + Debug + Copy>(
    encode: impl FnOnce(&mut BytesMut, T),
    decode: impl FnOnce(&mut Bytes) -> T,
    val: T,
) {
    let mut w = BytesMut::new();
    encode(&mut w, val);
    let mut b = w.freeze();
    assert_eq!(decode(&mut b), val);
    assert_eq!(b.remaining(), 0);
}

// Unsigned LEB128 round-trips

#[test]
fn vu16_roundtrip() {
    let vals = [0u16, 1, 127, 128, 255, 256, 16383, 16384, u16::MAX];
    for v in vals {
        roundtrip_unsigned(|w, n| w.put_vu16(n), |b| b.get_vu16(), v);
    }
}

#[test]
fn vu32_roundtrip() {
    let vals = [0u32, 1, 127, 128, 255, 256, 16383, 16384, 2097151, 2097152, u32::MAX];
    for v in vals {
        roundtrip_unsigned(|w, n| w.put_vu32(n), |b| b.get_vu32(), v);
    }
}

#[test]
fn vu64_roundtrip() {
    let vals = [
        0u64,
        1,
        127,
        128,
        255,
        256,
        16383,
        16384,
        2097151,
        2097152,
        268435455,
        268435456,
        u64::MAX,
    ];
    for v in vals {
        roundtrip_unsigned(|w, n| w.put_vu64(n), |b| b.get_vu64(), v);
    }
}

#[test]
fn vu128_roundtrip() {
    let vals = [
        0u128,
        1,
        127,
        128,
        255,
        256,
        16383,
        16384,
        2097151,
        2097152,
        268435455,
        268435456,
        u128::MAX,
    ];
    for v in vals {
        roundtrip_unsigned(|w, n| w.put_vu128(n), |b| b.get_vu128(), v);
    }
}

// ZigZag round-trips

#[test]
fn zz16_roundtrip() {
    let vals = [0i16, -1, 1, -2, 2, -64, 63, -65, 64, -8192, 8191, -8193, 8192, i16::MIN, i16::MAX];
    for v in vals {
        roundtrip_signed(|w, n| w.put_zz16(n), |b| b.get_zz16(), v);
    }
}

#[test]
fn zz32_roundtrip() {
    let vals = [
        0i32,
        -1,
        1,
        -2,
        2,
        -64,
        63,
        -65,
        64,
        -8192,
        8191,
        -134217728,
        134217727,
        i32::MIN,
        i32::MAX,
    ];
    for v in vals {
        roundtrip_signed(|w, n| w.put_zz32(n), |b| b.get_zz32(), v);
    }
}

#[test]
fn zz64_roundtrip() {
    let vals = [
        0i64,
        -1,
        1,
        -2,
        2,
        -64,
        63,
        -65,
        64,
        -8192,
        8191,
        -134217728,
        134217727,
        i64::MIN,
        i64::MAX,
    ];
    for v in vals {
        roundtrip_signed(|w, n| w.put_zz64(n), |b| b.get_zz64(), v);
    }
}

#[test]
fn zz128_roundtrip() {
    let vals = [
        0i128,
        -1,
        1,
        -2,
        2,
        -64,
        63,
        -65,
        64,
        -8192,
        8191,
        -134217728,
        134217727,
        i128::MIN,
        i128::MAX,
    ];
    for v in vals {
        roundtrip_signed(|w, n| w.put_zz128(n), |b| b.get_zz128(), v);
    }
}

// Known encoded byte sequences

#[test]
fn zz32_known_encodings() {
    // ZigZag: 0 → 0, -1 → 1, 1 → 2, -2 → 3, 2 → 4, ...
    let cases: &[(i32, &[u8])] = &[
        (0, &[0x00]),
        (-1, &[0x01]),
        (1, &[0x02]),
        (-2, &[0x03]),
        (2, &[0x04]),
        (-64, &[0x7F]),
        (63, &[0x7E]),
        (-65, &[0x81, 0x01]),
        (64, &[0x80, 0x01]),
        (-8192, &[0xFF, 0x7F]),
        (8191, &[0xFE, 0x7F]),
        (-134217728, &[0xFF, 0xFF, 0xFF, 0x7F]),
        (134217727, &[0xFE, 0xFF, 0xFF, 0x7F]),
    ];
    for &(val, expected) in cases {
        let mut w = BytesMut::new();
        w.put_zz32(val);
        assert_eq!(w.as_ref(), expected, "encoding {val}");

        let mut b = w.freeze();
        assert_eq!(b.get_zz32(), val, "decoding {val}");
    }
}

#[test]
fn vu32_known_encodings() {
    let cases: &[(u32, &[u8])] = &[
        (0, &[0x00]),
        (1, &[0x01]),
        (127, &[0x7F]),
        (128, &[0x80, 0x01]),
        (255, &[0xFF, 0x01]),
        (256, &[0x80, 0x02]),
        (16383, &[0xFF, 0x7F]),
        (16384, &[0x80, 0x80, 0x01]),
        (2097151, &[0xFF, 0xFF, 0x7F]),
        (2097152, &[0x80, 0x80, 0x80, 0x01]),
    ];
    for &(val, expected) in cases {
        let mut w = BytesMut::new();
        w.put_vu32(val);
        assert_eq!(w.as_ref(), expected, "encoding {val}");

        let mut b = w.freeze();
        assert_eq!(b.get_vu32(), val, "decoding {val}");
    }
}

// Underflow detection (try_get_* on short buffer)

#[test]
fn try_vu16_underflow() {
    // 2-byte encoded value, but only 1 byte in buffer
    let mut b = Bytes::from_static(&[0x80]);
    assert!(b.try_get_vu16().is_err());
}

#[test]
fn try_vu32_underflow() {
    // 2-byte encoded value, but only 1 byte in buffer
    let mut b = Bytes::from_static(&[0x80]);
    assert!(b.try_get_vu32().is_err());
}

#[test]
fn try_vu64_underflow() {
    let mut b = Bytes::from_static(&[0x80]);
    assert!(b.try_get_vu64().is_err());
}

#[test]
fn try_vu128_underflow() {
    let mut b = Bytes::from_static(&[0x80]);
    assert!(b.try_get_vu128().is_err());
}

#[test]
fn try_zz32_underflow() {
    let mut b = Bytes::from_static(&[0x80]);
    assert!(b.try_get_zz32().is_err());
}

#[test]
fn try_empty_buffer_underflow() {
    let mut b = Bytes::new();
    assert!(b.try_get_vu16().is_err());
    assert!(b.try_get_vu32().is_err());
    assert!(b.try_get_vu64().is_err());
    assert!(b.try_get_vu128().is_err());
    assert!(b.try_get_zz16().is_err());
    assert!(b.try_get_zz32().is_err());
    assert!(b.try_get_zz64().is_err());
    assert!(b.try_get_zz128().is_err());
}

// Overflow detection (try_get_* with too many continuation bytes)

#[test]
fn try_vu16_overflow() {
    // 3 bytes with all high bits set → value needs > 16 bits
    let mut b = Bytes::from_static(&[0xFF, 0xFF, 0xFF]);
    assert!(b.try_get_vu16().is_err());
}

#[test]
fn try_vu32_overflow() {
    // 6 bytes with all high bits set → value needs > 32 bits
    let mut b = Bytes::from_static(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    assert!(b.try_get_vu32().is_err());
}

#[test]
fn try_vu64_overflow() {
    // 11 bytes with all high bits set → value needs > 64 bits
    let mut b =
        Bytes::from_static(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    assert!(b.try_get_vu64().is_err());
}

#[test]
fn try_vu128_overflow() {
    // 20 bytes with all high bits set → value needs > 128 bits
    let mut b = Bytes::from_static(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ]);
    assert!(b.try_get_vu128().is_err());
}

#[test]
fn try_zz16_overflow() {
    let mut b = Bytes::from_static(&[0xFF, 0xFF, 0xFF]);
    assert!(b.try_get_zz16().is_err());
}

#[test]
fn try_zz32_overflow() {
    let mut b = Bytes::from_static(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    assert!(b.try_get_zz32().is_err());
}

#[test]
fn try_zz64_overflow() {
    let mut b =
        Bytes::from_static(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    assert!(b.try_get_zz64().is_err());
}

#[test]
fn try_zz128_overflow() {
    let mut b = Bytes::from_static(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ]);
    assert!(b.try_get_zz128().is_err());
}

// Panic tests (get_* on underflow/overflow)

#[test]
#[should_panic]
fn get_vu16_empty_panics() {
    let mut b = Bytes::new();
    let _ = b.get_vu16();
}

#[test]
#[should_panic]
fn get_vu32_empty_panics() {
    let mut b = Bytes::new();
    let _ = b.get_vu32();
}

#[test]
#[should_panic]
fn get_vu64_empty_panics() {
    let mut b = Bytes::new();
    let _ = b.get_vu64();
}

#[test]
#[should_panic]
fn get_vu128_empty_panics() {
    let mut b = Bytes::new();
    let _ = b.get_vu128();
}

#[test]
#[should_panic]
fn get_vu16_overflow_panics() {
    let mut b = Bytes::from_static(&[0xFF, 0xFF, 0xFF]);
    let _ = b.get_vu16();
}

#[test]
#[should_panic]
fn get_vu32_overflow_panics() {
    let mut b = Bytes::from_static(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    let _ = b.get_vu32();
}

#[test]
#[should_panic]
fn get_vu64_overflow_panics() {
    let mut b =
        Bytes::from_static(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    let _ = b.get_vu64();
}

#[test]
#[should_panic]
fn get_vu128_overflow_panics() {
    let mut b = Bytes::from_static(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ]);
    let _ = b.get_vu128();
}

// Canonical encoding (no leading zero bytes)

#[test]
fn vu32_canonical_no_excess_bytes() {
    // 128 encodes as 2 bytes [0x80, 0x01], not 3
    let mut w = BytesMut::new();
    w.put_vu32(128);
    assert_eq!(w.len(), 2);

    // 16384 encodes as 3 bytes [0x80, 0x80, 0x01], not 4
    let mut w = BytesMut::new();
    w.put_vu32(16384);
    assert_eq!(w.len(), 3);
}

#[test]
fn vu64_canonical_no_excess_bytes() {
    // 2097152 encodes as 4 bytes
    let mut w = BytesMut::new();
    w.put_vu64(2097152);
    assert_eq!(w.len(), 4);

    // 268435456 encodes as 5 bytes
    let mut w = BytesMut::new();
    w.put_vu64(268435456);
    assert_eq!(w.len(), 5);
}

// BytesMut round-trip (encode + decode in place)

#[test]
fn vu32_bytesmut_roundtrip() {
    let mut buf = BytesMut::new();
    buf.put_vu32(42);
    buf.put_vu32(0);
    buf.put_vu32(u32::MAX);
    let mut b = buf.freeze();
    assert_eq!(b.get_vu32(), 42);
    assert_eq!(b.get_vu32(), 0);
    assert_eq!(b.get_vu32(), u32::MAX);
    assert_eq!(b.remaining(), 0);
}

#[test]
fn zz64_bytesmut_roundtrip() {
    let mut buf = BytesMut::new();
    buf.put_zz64(0);
    buf.put_zz64(-1);
    buf.put_zz64(1);
    buf.put_zz64(i64::MIN);
    buf.put_zz64(i64::MAX);
    let mut b = buf.freeze();
    assert_eq!(b.get_zz64(), 0);
    assert_eq!(b.get_zz64(), -1);
    assert_eq!(b.get_zz64(), 1);
    assert_eq!(b.get_zz64(), i64::MIN);
    assert_eq!(b.get_zz64(), i64::MAX);
    assert_eq!(b.remaining(), 0);
}
