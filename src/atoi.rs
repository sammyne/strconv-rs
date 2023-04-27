use crate::{NumError, NumErrorCause};

/// Interprets a string s in the given base (0, 2 to 36) and
/// bit size (0 to 64) and returns the corresponding value i.
///
/// The string may begin with a leading sign: "+" or "-".
///
/// If the base argument is 0, the true base is implied by the string's
/// prefix following the sign (if present): 2 for "0b", 8 for "0" or "0o",
/// 16 for "0x", and 10 otherwise. Also, for argument base 0 only,
/// underscore characters are permitted as defined by the Go syntax for
/// [integer literals].
///
/// The `bit_size` argument specifies the integer type
/// that the result must fit into. Bit sizes 0, 8, 16, 32, and 64
/// correspond to int, int8, int16, int32, and int64.
/// If `bit_size` is below 0 or above 64, an error is returned.
///
/// The errors that `parse_int` returns have concrete type [NumError][crate::NumError]
/// and include `err.num = s`. If `s` is empty or contains invalid
/// digits, `err.err` = [NumErrorCause::InvalidSyntax][crate::NumErrorCause::InvalidSyntax] and the returned value is 0;
/// if the value corresponding to s cannot be represented by a
/// signed integer of the given size, `err.err` = [NumErrorCause::OutOfRangeSigned][crate::NumErrorCause::OutOfRangeSigned]
/// and the returned value is the maximum magnitude integer of the
/// appropriate `bit_size` and sign.
///
/// [integer literals]: https://go.dev/ref/spec#Integer_literals
pub fn parse_int(s: &'static str, base: u8, bit_size: u8) -> Result<i64, NumError> {
    const FN_PARSE_INT: &'static str = "parse_int";

    if s.is_empty() {
        return Err(NumError::syntax(FN_PARSE_INT, s));
    }

    let s0 = s;
    let neg = s0.starts_with('-');
    // Pick off leading sign.
    let s = s.strip_prefix(|c| (c == '+') || (c == '-')).unwrap_or(s);

    // Convert unsigned and check range.
    let un = match parse_uint(s, base, bit_size) {
        Err(mut err) => match err.err {
            NumErrorCause::OutOfRangeUnsigned { bound_hint } => bound_hint,
            _ => {
                err.func = FN_PARSE_INT.to_string();
                err.num = s0.to_string();
                return Err(err);
            }
        },
        Ok(v) => v,
    };

    let mut bit_size = bit_size as u32;
    if bit_size == 0 {
        bit_size = usize::BITS;
    }

    let cutoff = 1u64 << (bit_size - 1);
    if !neg && (un >= cutoff) {
        let err = NumError::range_signed(FN_PARSE_INT, s0, (cutoff - 1) as i64);
        return Err(err);
    }
    if neg && (un > cutoff) {
        let hint = if cutoff == (i64::MIN as u64) {
            i64::MIN
        } else {
            -(cutoff as i64)
        };
        return Err(NumError::range_signed(FN_PARSE_INT, s0, hint));
    }

    let mut n = un as i64;
    if neg && (n != i64::MIN) {
        n = -n;
    }

    Ok(n)
}

/// Like [parse_int] but for unsigned numbers.
///
/// A sign prefix is not permitted.
pub fn parse_uint(s: &'static str, base: u8, bit_size: u8) -> Result<u64, NumError> {
    const FN_PARSE_UINT: &'static str = "parse_uint";

    if s.is_empty() {
        return Err(NumError::syntax(FN_PARSE_UINT, s));
    }

    let base0 = base == 0;
    let s0 = s;
    let (s, base) = match base {
        2..=36 => (s.as_bytes(), base), // valid base; nothing to do
        0 => {
            // Look for octal, hex prefix.
            let mut b = 10;
            let mut s = s.as_bytes();
            if s[0] == b'0' {
                if (s.len() >= 3) && (s[1].to_ascii_lowercase() == b'b') {
                    b = 2;
                    s = &s[2..];
                } else if (s.len() >= 3) && (s[1].to_ascii_lowercase() == b'o') {
                    b = 8;
                    s = &s[2..];
                } else if (s.len() >= 3) && (s[1].to_ascii_lowercase() == b'x') {
                    b = 16;
                    s = &s[2..];
                } else {
                    b = 8;
                    s = &s[1..];
                }
            }

            (s, b)
        }
        _ => return Err(NumError::base(FN_PARSE_UINT, s0, base)),
    };

    let bit_size = if bit_size == 0 {
        usize::BITS as u8
    } else if bit_size > 64 {
        return Err(NumError::bit_size(FN_PARSE_UINT, s0, bit_size));
    } else {
        bit_size
    };

    // Cutoff is the smallest number such that cutoff*base > u64::MAX.
    // Use compile-time constants for common cases.
    let cutoff = match base {
        10 => u64::MAX / 10 + 1,
        16 => u64::MAX / 16 + 1,
        _ => u64::MAX / (base as u64) + 1,
    };

    let max_val = if bit_size == 64 {
        u64::MAX
    } else {
        (1u64 << (bit_size as usize)) - 1
    };

    let mut underscores = false;
    let mut n = 0u64;
    for &c in s {
        let d = match c {
            b'_' if base0 => {
                underscores = true;
                continue;
            }
            b'0'..=b'9' => c - b'0',
            b'a'..=b'z' => c - b'a' + 10,
            b'A'..=b'Z' => c - b'A' + 10,
            _ => return Err(NumError::syntax(FN_PARSE_UINT, s0)),
        };

        if d >= base {
            return Err(NumError::syntax(FN_PARSE_UINT, s0));
        }

        if n >= cutoff {
            // n*base overflows
            return Err(NumError::range_unsigned(FN_PARSE_UINT, s0, max_val));
        }
        n *= base as u64;

        let n1 = n.wrapping_add(d as u64);
        if (n1 < n) || (n1 > max_val) {
            // n+d overflows
            return Err(NumError::range_unsigned(FN_PARSE_UINT, s0, max_val));
        }
        n = n1;
    }

    if underscores && !underscore_ok(s0) {
        return Err(NumError::syntax(FN_PARSE_UINT, s0));
    }

    Ok(n)
}

fn underscore_ok(s: &str) -> bool {
    let mut saw = b'^';
    let mut i = 0;
    let mut s = s.as_bytes();

    if (s.len() >= 1) && ((s[0] == b'-') || (s[0] == b'+')) {
        s = &s[1..];
    }

    let mut hex = false;
    if (s.len() >= 2)
        && (s[0] == b'0')
        && ((s[1].to_ascii_lowercase() == b'b')
            || (s[1].to_ascii_lowercase() == b'o')
            || (s[1].to_ascii_lowercase() == b'x'))
    {
        i = 2;
        saw = b'0';
        hex = s[1].to_ascii_lowercase() == b'x';
    }

    for j in i..s.len() {
        let c = s[j];

        if ((b'0' <= c) && (c <= b'9')) || (hex && (b'a' <= c) && (c <= b'f')) {
            saw = b'0';
            continue;
        }

        if c == b'_' {
            if saw != b'0' {
                return false;
            }
            saw = b'_';
            continue;
        }

        if saw == b'_' {
            return false;
        }

        saw = b'!';
    }

    saw != b'_'
}
