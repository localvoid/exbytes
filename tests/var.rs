use bytes::BytesMut;
use exbytes::{BytesExt as _, BytesMutExt as _};

#[test]
fn put_ivar_0() {
    let mut w = BytesMut::new();
    w.put_zz32(0);
    let mut b = w.freeze();
    assert_eq!(b.len(), 1);
    let bytes = b.get(..1).unwrap();
    assert_eq!(bytes, &[0]);
    assert_eq!(b.get_zz32(), 0);
}

#[test]
fn put_ivar_neg_64() {
    let mut w = BytesMut::new();
    w.put_zz32(-64);
    let mut b = w.freeze();
    assert_eq!(b.len(), 1);
    let bytes = b.get(..1).unwrap();
    assert_eq!(bytes, &[127]);
    assert_eq!(b.get_zz32(), -64);
}

#[test]
fn put_ivar_63() {
    let mut w = BytesMut::new();
    w.put_zz32(63);
    let mut b = w.freeze();
    assert_eq!(b.len(), 1);
    let bytes = b.get(..1).unwrap();
    assert_eq!(bytes, &[126]);
    assert_eq!(b.get_zz32(), 63);
}

#[test]
fn put_ivar_neg_65() {
    let mut w = BytesMut::new();
    w.put_zz32(-65);
    let mut b = w.freeze();
    assert_eq!(b.len(), 2);
    let bytes = b.get(..2).unwrap();
    assert_eq!(bytes, &[129, 1]);
    assert_eq!(b.get_zz32(), -65);
}

#[test]
fn put_ivar_64() {
    let mut w = BytesMut::new();
    w.put_zz32(64);
    let mut b = w.freeze();
    assert_eq!(b.len(), 2);
    let bytes = b.get(..2).unwrap();
    assert_eq!(bytes, &[128, 1]);
    assert_eq!(b.get_zz32(), 64);
}

#[test]
fn put_ivar_neg_8192() {
    let mut w = BytesMut::new();
    w.put_zz32(-8192);
    let mut b = w.freeze();
    assert_eq!(b.len(), 2);
    let bytes = b.get(..2).unwrap();
    assert_eq!(bytes, &[255, 127]);
    assert_eq!(b.get_zz32(), -8192);
}

#[test]
fn put_ivar_8191() {
    let mut w = BytesMut::new();
    w.put_zz32(8191);
    let mut b = w.freeze();
    assert_eq!(b.len(), 2);
    let bytes = b.get(..2).unwrap();
    assert_eq!(bytes, &[254, 127]);
    assert_eq!(b.get_zz32(), 8191);
}

#[test]
fn put_ivar_neg_134217728() {
    let mut w = BytesMut::new();
    w.put_zz32(-134217728);
    let mut b = w.freeze();
    assert_eq!(b.len(), 4);
    let bytes = b.get(..4).unwrap();
    assert_eq!(bytes, &[255, 255, 255, 127]);
    assert_eq!(b.get_zz32(), -134217728);
}

#[test]
fn put_ivar_134217727() {
    let mut w = BytesMut::new();
    w.put_zz32(134217727);
    let mut b = w.freeze();
    assert_eq!(b.len(), 4);
    let bytes = b.get(..4).unwrap();
    assert_eq!(bytes, &[254, 255, 255, 127]);
    assert_eq!(b.get_zz32(), 134217727);
}
