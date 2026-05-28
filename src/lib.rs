//! Extensions for [`Bytes`] and [`BytesMut`] providing variable-length integer encoding
//! (unsigned LEB128 and signed ZigZag).
//!
//! # LEB128
//!
//! Unsigned LEB128 encodes an integer in 7-bit groups, most significant group first,
//! with the high bit set on every byte except the last.
//!
//! # ZigZag
//!
//! ZigZag maps signed integers to unsigned LEB128 by interleaving positive and negative
//! values: `0 → 0, -1 → 1, 1 → 2, -2 → 3, …`, so that small absolute values produce
//! small encoded outputs.
use bytes::{Buf, BufMut, Bytes, BytesMut, TryGetError};

/// Errors that can occur when decoding a LEB128 value.
#[derive(Debug)]
pub enum TryGetLebError {
    /// The buffer ran out of data before a complete value could be decoded.
    Underflow,
    /// The decoded value exceeds the bit-width of the target integer type.
    LebOverflow,
}

impl std::fmt::Display for TryGetLebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Underflow => f.write_str("Buffer underflow: not enough data to decode value"),
            Self::LebOverflow => {
                f.write_str("LEB overflow: value exceeds the bit-width of the target type")
            }
        }
    }
}

impl std::error::Error for TryGetLebError {}

impl From<TryGetError> for TryGetLebError {
    fn from(_e: TryGetError) -> Self {
        Self::Underflow
    }
}

impl From<TryGetLebError> for std::io::Error {
    fn from(error: TryGetLebError) -> Self {
        std::io::Error::other(error)
    }
}

/// Extension trait providing LEB128 and ZigZag decoding on [`Bytes`] / [`BytesMut`].
///
/// The `get_*` methods **panic** on insufficient data or overflow; use `try_get_*` for
/// fallible alternatives.
pub trait BytesExt {
    /// Decodes an unsigned LEB128 `u16`, advancing the internal cursor.
    ///
    /// # Panics
    ///
    /// Panics if the buffer is exhausted or the value exceeds 16 bits.
    fn get_vu16(&mut self) -> u16;
    /// Decodes an unsigned LEB128 `u32`, advancing the internal cursor.
    ///
    /// # Panics
    ///
    /// Panics if the buffer is exhausted or the value exceeds 32 bits.
    fn get_vu32(&mut self) -> u32;
    /// Decodes an unsigned LEB128 `u64`, advancing the internal cursor.
    ///
    /// # Panics
    ///
    /// Panics if the buffer is exhausted or the value exceeds 64 bits.
    fn get_vu64(&mut self) -> u64;
    /// Decodes an unsigned LEB128 `u128`, advancing the internal cursor.
    ///
    /// # Panics
    ///
    /// Panics if the buffer is exhausted or the value exceeds 128 bits.
    fn get_vu128(&mut self) -> u128;

    /// Tries to decode an unsigned LEB128 `u16`.
    ///
    /// # Errors
    ///
    /// Returns [`TryGetLebError::Underflow`] if the buffer is exhausted, or
    /// [`TryGetLebError::LebOverflow`] if the value exceeds 16 bits.
    fn try_get_vu16(&mut self) -> Result<u16, TryGetLebError>;
    /// Tries to decode an unsigned LEB128 `u32`.
    ///
    /// # Errors
    ///
    /// Returns [`TryGetLebError::Underflow`] if the buffer is exhausted, or
    /// [`TryGetLebError::LebOverflow`] if the value exceeds 32 bits.
    fn try_get_vu32(&mut self) -> Result<u32, TryGetLebError>;
    /// Tries to decode an unsigned LEB128 `u64`.
    ///
    /// # Errors
    ///
    /// Returns [`TryGetLebError::Underflow`] if the buffer is exhausted, or
    /// [`TryGetLebError::LebOverflow`] if the value exceeds 64 bits.
    fn try_get_vu64(&mut self) -> Result<u64, TryGetLebError>;
    /// Tries to decode an unsigned LEB128 `u128`.
    ///
    /// # Errors
    ///
    /// Returns [`TryGetLebError::Underflow`] if the buffer is exhausted, or
    /// [`TryGetLebError::LebOverflow`] if the value exceeds 128 bits.
    fn try_get_vu128(&mut self) -> Result<u128, TryGetLebError>;

    /// Decodes a ZigZag-encoded signed `i16`, advancing the internal cursor.
    ///
    /// # Panics
    ///
    /// Panics if the underlying LEB128 decode panics.
    fn get_zz16(&mut self) -> i16;
    /// Decodes a ZigZag-encoded signed `i32`, advancing the internal cursor.
    ///
    /// # Panics
    ///
    /// Panics if the underlying LEB128 decode panics.
    fn get_zz32(&mut self) -> i32;
    /// Decodes a ZigZag-encoded signed `i64`, advancing the internal cursor.
    ///
    /// # Panics
    ///
    /// Panics if the underlying LEB128 decode panics.
    fn get_zz64(&mut self) -> i64;
    /// Decodes a ZigZag-encoded signed `i128`, advancing the internal cursor.
    ///
    /// # Panics
    ///
    /// Panics if the underlying LEB128 decode panics.
    fn get_zz128(&mut self) -> i128;

    /// Tries to decode a ZigZag-encoded signed `i16`.
    ///
    /// # Errors
    ///
    /// Propagates the error from the underlying LEB128 decode.
    fn try_get_zz16(&mut self) -> Result<i16, TryGetLebError>;
    /// Tries to decode a ZigZag-encoded signed `i32`.
    ///
    /// # Errors
    ///
    /// Propagates the error from the underlying LEB128 decode.
    fn try_get_zz32(&mut self) -> Result<i32, TryGetLebError>;
    /// Tries to decode a ZigZag-encoded signed `i64`.
    ///
    /// # Errors
    ///
    /// Propagates the error from the underlying LEB128 decode.
    fn try_get_zz64(&mut self) -> Result<i64, TryGetLebError>;
    /// Tries to decode a ZigZag-encoded signed `i128`.
    ///
    /// # Errors
    ///
    /// Propagates the error from the underlying LEB128 decode.
    fn try_get_zz128(&mut self) -> Result<i128, TryGetLebError>;
}

/// Internal: panicking LEB128 decode loop.
macro_rules! decode_leb {
    ($buf:expr, $t:ty, $bits:expr) => {{
        let mut res: $t = 0;
        let mut shift = 0;
        loop {
            let byte = $buf.get_u8();
            res |= ((byte & 0x7F) as $t) << shift;
            if (byte & 0x80) == 0 {
                return res;
            }
            shift += 7;
            if shift >= $bits + 7 {
                panic!("overflow");
            }
        }
    }};
}

/// Internal: fallible LEB128 decode loop.
macro_rules! try_decode_leb {
    ($buf:expr, $t:ty, $bits:expr) => {{
        let mut res: $t = 0;
        let mut shift = 0;
        loop {
            let byte = $buf.try_get_u8().map_err(TryGetLebError::from)?;
            res |= ((byte & 0x7F) as $t) << shift;
            if (byte & 0x80) == 0 {
                return Ok(res);
            }
            shift += 7;
            if shift >= $bits + 7 {
                return Err(TryGetLebError::LebOverflow);
            }
        }
    }};
}
impl BytesExt for Bytes {
    #[inline]
    fn get_zz16(&mut self) -> i16 {
        let n = self.get_vu16();
        ((n >> 1) as i16) ^ -((n & 1) as i16)
    }
    #[inline]
    fn get_zz32(&mut self) -> i32 {
        let n = self.get_vu32();
        ((n >> 1) as i32) ^ -((n & 1) as i32)
    }
    #[inline]
    fn get_zz64(&mut self) -> i64 {
        let n = self.get_vu64();
        ((n >> 1) as i64) ^ -((n & 1) as i64)
    }
    #[inline]
    fn get_zz128(&mut self) -> i128 {
        let n = self.get_vu128();
        ((n >> 1) as i128) ^ -((n & 1) as i128)
    }

    #[inline]
    fn try_get_zz16(&mut self) -> Result<i16, TryGetLebError> {
        let n = self.try_get_vu16()?;
        Ok(((n >> 1) as i16) ^ -((n & 1) as i16))
    }

    #[inline]
    fn try_get_zz32(&mut self) -> Result<i32, TryGetLebError> {
        let n = self.try_get_vu32()?;
        Ok(((n >> 1) as i32) ^ -((n & 1) as i32))
    }

    #[inline]
    fn try_get_zz64(&mut self) -> Result<i64, TryGetLebError> {
        let n = self.try_get_vu64()?;
        Ok(((n >> 1) as i64) ^ -((n & 1) as i64))
    }

    #[inline]
    fn try_get_zz128(&mut self) -> Result<i128, TryGetLebError> {
        let n = self.try_get_vu128()?;
        Ok(((n >> 1) as i128) ^ -((n & 1) as i128))
    }

    #[inline]
    fn get_vu16(&mut self) -> u16 {
        decode_leb!(self, u16, 16)
    }
    #[inline]
    fn get_vu32(&mut self) -> u32 {
        decode_leb!(self, u32, 32)
    }
    #[inline]
    fn get_vu64(&mut self) -> u64 {
        decode_leb!(self, u64, 64)
    }
    #[inline]
    fn get_vu128(&mut self) -> u128 {
        decode_leb!(self, u128, 128)
    }

    #[inline]
    fn try_get_vu16(&mut self) -> Result<u16, TryGetLebError> {
        try_decode_leb!(self, u16, 16)
    }
    #[inline]
    fn try_get_vu32(&mut self) -> Result<u32, TryGetLebError> {
        try_decode_leb!(self, u32, 32)
    }
    #[inline]
    fn try_get_vu64(&mut self) -> Result<u64, TryGetLebError> {
        try_decode_leb!(self, u64, 64)
    }
    #[inline]
    fn try_get_vu128(&mut self) -> Result<u128, TryGetLebError> {
        try_decode_leb!(self, u128, 128)
    }
}

/// Extension trait providing LEB128 and ZigZag encoding on [`BytesMut`].
pub trait BytesMutExt {
    /// Encodes `n` as unsigned LEB128 and appends it to the buffer.
    fn put_vu16(&mut self, n: u16);
    /// Encodes `n` as unsigned LEB128 and appends it to the buffer.
    fn put_vu32(&mut self, n: u32);
    /// Encodes `n` as unsigned LEB128 and appends it to the buffer.
    fn put_vu64(&mut self, n: u64);
    /// Encodes `n` as unsigned LEB128 and appends it to the buffer.
    fn put_vu128(&mut self, n: u128);

    /// Encodes `n` as ZigZag then unsigned LEB128 and appends it to the buffer.
    fn put_zz16(&mut self, n: i16);
    /// Encodes `n` as ZigZag then unsigned LEB128 and appends it to the buffer.
    fn put_zz32(&mut self, n: i32);
    /// Encodes `n` as ZigZag then unsigned LEB128 and appends it to the buffer.
    fn put_zz64(&mut self, n: i64);
    /// Encodes `n` as ZigZag then unsigned LEB128 and appends it to the buffer.
    fn put_zz128(&mut self, n: i128);
}

/// Internal: LEB128 encode loop.
macro_rules! encode_leb {
    ($buf:expr, $n:expr, $t:ty) => {{
        let mut val = $n;
        loop {
            if val < 0x80 {
                $buf.put_u8(val as u8);
                break;
            } else {
                $buf.put_u8(((val & 0x7F) | 0x80) as u8);
                val >>= 7;
            }
        }
    }};
}

impl BytesMutExt for BytesMut {
    // LEB128 Encoding
    #[inline]
    fn put_vu16(&mut self, n: u16) {
        encode_leb!(self, n, u16);
    }
    #[inline]
    fn put_vu32(&mut self, n: u32) {
        encode_leb!(self, n, u32);
    }
    #[inline]
    fn put_vu64(&mut self, n: u64) {
        encode_leb!(self, n, u64);
    }
    #[inline]
    fn put_vu128(&mut self, n: u128) {
        encode_leb!(self, n, u128);
    }

    // ZigZag Encoding
    #[inline]
    fn put_zz16(&mut self, n: i16) {
        self.put_vu16(((n << 1) ^ (n >> 15)) as u16);
    }
    #[inline]
    fn put_zz32(&mut self, n: i32) {
        self.put_vu32(((n << 1) ^ (n >> 31)) as u32);
    }
    #[inline]
    fn put_zz64(&mut self, n: i64) {
        self.put_vu64(((n << 1) ^ (n >> 63)) as u64);
    }
    #[inline]
    fn put_zz128(&mut self, n: i128) {
        self.put_vu128(((n << 1) ^ (n >> 127)) as u128);
    }
}
