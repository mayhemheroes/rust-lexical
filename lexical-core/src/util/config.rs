//! Config settings for lexical.

use super::rounding::RoundingKind;

// GLOBALS

/// Not a Number literal
///
/// To change the expected representation of NaN as a string,
/// change this value during before using lexical.
///
/// # Warnings
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut NAN_STRING: &str = "NaN";

/// Short infinity literal
///
/// To change the expected representation of Infinity as a string,
/// change this value during before using lexical.
///
/// # Warnings
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut INF_STRING: &str = "inf";

/// Long infinity literal
///
/// To change the expected backup representation of Infinity as a string,
/// change this value during before using lexical.
///
/// # Warnings
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut INFINITY_STRING: &str = "infinity";

/// Default character for scientific notation, used when the radix < 15.
///
/// To change the expected, default character for an exponent,
/// change this value during before using lexical.
///
/// # Warnings
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut EXPONENT_DEFAULT_CHAR: u8 = b'e';

/// Backup character for scientific notation, used when the radix >= 15.
///
/// For numerical strings of radix >= 15, 'e' or 'E' is a valid digit,
/// and therefore may no longer be used as a marker for the exponent.
///
/// To change the expected, default character for an exponent,
/// change this value during before using lexical.
///
/// # Warnings
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut EXPONENT_BACKUP_CHAR: u8 = b'^';

// CONSTANTS

// Simple, fast optimization.
// Since we're declaring a variable on the stack, and our power-of-two
// alignment dramatically improved atoi performance, do it.
cfg_if! {
if #[cfg(feature = "radix")] {
    // Use 256, actually, since we seem to have memory issues with f64.
    // Clearly not sufficient memory allocated for non-base10 values.

    /// The minimum buffer size required to serialize any i8 value.
    pub const MAX_I8_SIZE: usize = 16;

    /// The minimum buffer size required to serialize any i16 value.
    pub const MAX_I16_SIZE: usize = 32;

    /// The minimum buffer size required to serialize any i32 value.
    pub const MAX_I32_SIZE: usize = 64;

    /// The minimum buffer size required to serialize any i64 value.
    pub const MAX_I64_SIZE: usize = 128;

    /// The minimum buffer size required to serialize any i128 value.
    pub const MAX_I128_SIZE: usize = 256;

    /// The minimum buffer size required to serialize any u8 value.
    pub const MAX_U8_SIZE: usize = 16;

    /// The minimum buffer size required to serialize any u16 value.
    pub const MAX_U16_SIZE: usize = 32;

    /// The minimum buffer size required to serialize any u32 value.
    pub const MAX_U32_SIZE: usize = 64;

    /// The minimum buffer size required to serialize any u64 value.
    pub const MAX_U64_SIZE: usize = 128;

    /// The minimum buffer size required to serialize any u128 value.
    pub const MAX_U128_SIZE: usize = 256;

    /// The minimum buffer size required to serialize any f32 value.
    pub const MAX_F32_SIZE: usize = 256;

    /// The minimum buffer size required to serialize any f64 value.
    pub const MAX_F64_SIZE: usize = 256;
} else {
    // The f64 buffer is actually a size of 60, but use 64 since it's a
    // power of 2.

    /// The minimum buffer size required to serialize any i8 value.
    pub const MAX_I8_SIZE: usize = 4;

    /// The minimum buffer size required to serialize any i16 value.
    pub const MAX_I16_SIZE: usize = 6;

    /// The minimum buffer size required to serialize any i32 value.
    pub const MAX_I32_SIZE: usize = 11;

    /// The minimum buffer size required to serialize any i64 value.
    pub const MAX_I64_SIZE: usize = 20;

    /// The minimum buffer size required to serialize any i128 value.
    pub const MAX_I128_SIZE: usize = 40;

    /// The minimum buffer size required to serialize any u8 value.
    pub const MAX_U8_SIZE: usize = 3;

    /// The minimum buffer size required to serialize any u16 value.
    pub const MAX_U16_SIZE: usize = 5;

    /// The minimum buffer size required to serialize any u32 value.
    pub const MAX_U32_SIZE: usize = 10;

    /// The minimum buffer size required to serialize any u64 value.
    pub const MAX_U64_SIZE: usize = 20;

    /// The minimum buffer size required to serialize any u128 value.
    pub const MAX_U128_SIZE: usize = 39;

    /// The minimum buffer size required to serialize any f32 value.
    pub const MAX_F32_SIZE: usize = 64;

    /// The minimum buffer size required to serialize any f64 value.
    pub const MAX_F64_SIZE: usize = 64;
}} // cfg_if

cfg_if! {
if #[cfg(target_pointer_width = "16")] {
    /// The minimum buffer size required to serialize any isize value.
    pub const MAX_ISIZE_SIZE: usize = MAX_I16_SIZE;

    /// The minimum buffer size required to serialize any usize value.
    pub const MAX_USIZE_SIZE: usize = MAX_U16_SIZE;
} else if #[cfg(target_pointer_width = "32")] {
    /// The minimum buffer size required to serialize any isize value.
    pub const MAX_ISIZE_SIZE: usize = MAX_I32_SIZE;

    /// The minimum buffer size required to serialize any usize value.
    pub const MAX_USIZE_SIZE: usize = MAX_U32_SIZE;
} else if #[cfg(target_pointer_width = "64")] {
    /// The minimum buffer size required to serialize any isize value.
    pub const MAX_ISIZE_SIZE: usize = MAX_I64_SIZE;

    /// The minimum buffer size required to serialize any usize value.
    pub const MAX_USIZE_SIZE: usize = MAX_U64_SIZE;
}}  // cfg_if

/// The maximum number of bytes that any number-to-string function may write.
pub const BUFFER_SIZE: usize = MAX_F64_SIZE;

/// The rounding scheme for float conversions.
///
/// This defines the global rounding-scheme for float parsing operations.
/// By default, this is set to `RoundingKind::NearestTieEven`. IEEE754
/// recommends this as the default for all for decimal and binary
/// operations.
///
/// # Warnings
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut FLOAT_ROUNDING: RoundingKind = RoundingKind::NearestTieEven;

// FUNCTIONS

/// Get the exponent notation character.
#[inline]
pub(crate) extern "C" fn exponent_notation_char(radix: u32)
    -> u8
{
    unsafe {
        if radix >= 15 { EXPONENT_BACKUP_CHAR } else { EXPONENT_DEFAULT_CHAR }
    }
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use atof::*;
    use ftoa::*;
    use util::*;
    use util::test::*;
    use super::*;

    #[test]
    fn exponent_notation_char_test() {
        unsafe {
            assert_eq!(exponent_notation_char(2), EXPONENT_DEFAULT_CHAR);
            assert_eq!(exponent_notation_char(8), EXPONENT_DEFAULT_CHAR);
            assert_eq!(exponent_notation_char(10), EXPONENT_DEFAULT_CHAR);
            assert_eq!(exponent_notation_char(15), EXPONENT_BACKUP_CHAR);
            assert_eq!(exponent_notation_char(16), EXPONENT_BACKUP_CHAR);
            assert_eq!(exponent_notation_char(32), EXPONENT_BACKUP_CHAR);
        }
    }

    // Only enable when no other threads touch NAN_STRING or INFINITY_STRING.
    #[test]
    #[ignore]
    fn special_bytes_test() {
        unsafe {
            let mut buffer = new_buffer();
            // Test serializing and deserializing special strings.
            assert!(try_atof32_slice(10, b"NaN").value.is_nan());
            assert!(try_atof32_slice(10, b"nan").value.is_nan());
            assert!(try_atof32_slice(10, b"NAN").value.is_nan());
            assert!(try_atof32_slice(10, b"inf").value.is_infinite());
            assert!(try_atof32_slice(10, b"INF").value.is_infinite());
            assert!(try_atof32_slice(10, b"Infinity").value.is_infinite());
            assert_eq!(f64toa_slice(f64::NAN, 10, &mut buffer), b"NaN");
            assert_eq!(f64toa_slice(f64::INFINITY, 10, &mut buffer), b"inf");

            NAN_STRING = "nan";
            INF_STRING = "Infinity";

            assert!(try_atof32_slice(10, b"inf").error.code == ErrorCode::InvalidDigit);
            assert!(try_atof32_slice(10, b"Infinity").value.is_infinite());
            assert_eq!(f64toa_slice(f64::NAN, 10, &mut buffer), b"nan");
            assert_eq!(f64toa_slice(f64::INFINITY, 10, &mut buffer), b"Infinity");

            NAN_STRING = "NaN";
            INF_STRING = "inf";
        }
    }

    // Only enable when no other threads touch FLOAT_ROUNDING.
    #[test]
    #[ignore]
    fn special_rounding_test() {
        // Each one of these pairs is halfway, and we can detect the
        // rounding schemes from this.
        unsafe {
            // Nearest, tie-even
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740996.0);

            // Nearest, tie-away-zero
            FLOAT_ROUNDING = RoundingKind::NearestTieAwayZero;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740996.0);

            // Toward positive infinity
            FLOAT_ROUNDING = RoundingKind::TowardPositiveInfinity;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740996.0);

            // Toward negative infinity
            FLOAT_ROUNDING = RoundingKind::TowardNegativeInfinity;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740994.0);

            // Toward zero
            FLOAT_ROUNDING = RoundingKind::TowardZero;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740994.0);

            // Reset to default
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
        }
    }
}
