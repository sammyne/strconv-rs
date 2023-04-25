use strconv::{NumError, NumErrorCause};

//type OpaqueError = Box<dyn std::error::Error + Sync + Send + 'static>;

lazy_static::lazy_static! {
  static ref PARSE_INT32_TESTS: Vec<ParseInt32Test> = vec![
    ParseInt32Test::err_signed("", 0, NumErrorCause::Syntax),
    ParseInt32Test::ok("0", 0),
    ParseInt32Test::ok("-0", 0),
    ParseInt32Test::ok("1", 1),
    ParseInt32Test::ok("-1", -1),
    ParseInt32Test::ok("12345", 12345),
    ParseInt32Test::ok("-12345", -12345),
    ParseInt32Test::ok("012345", 12345),
    ParseInt32Test::ok("-012345", -12345),
    ParseInt32Test::err_signed("12345x", 0, NumErrorCause::Syntax),
    ParseInt32Test::err_signed("-12345x", 0, NumErrorCause::Syntax),
    ParseInt32Test::ok("987654321", 987654321),
    ParseInt32Test::ok("-987654321", -987654321),
    ParseInt32Test::ok("2147483647", i32::MAX),
    ParseInt32Test::ok("-2147483647", -i32::MAX),
    ParseInt32Test::err_signed("2147483648", i32::MAX, NumErrorCause::RangeSigned{bound_hint: 2147483647}),
    ParseInt32Test::ok("-2147483648", i32::MIN),
    ParseInt32Test::err_signed("2147483649", i32::MAX, NumErrorCause::RangeSigned{bound_hint: 2147483647}),
    ParseInt32Test::err_signed("-2147483649", i32::MIN, NumErrorCause::RangeSigned{bound_hint: -2147483648}),
    ParseInt32Test::err_signed("-1_2_3_4_5", 0, NumErrorCause::Syntax), // base=10 so no underscores allowed
    ParseInt32Test::err_signed("-_12345", 0, NumErrorCause::Syntax),
    ParseInt32Test::err_signed("_12345", 0, NumErrorCause::Syntax),
    ParseInt32Test::err_signed("1__2345", 0, NumErrorCause::Syntax),
    ParseInt32Test::err_signed("12345_", 0, NumErrorCause::Syntax),
  ];

  static ref PARSE_INT64_TESTS: Vec<ParseInt64Test> = vec![
    ParseInt64Test::err_signed("", 0, NumErrorCause::Syntax),
    ParseInt64Test::ok("0", 0),
    ParseInt64Test::ok("-0", 0),
    ParseInt64Test::ok("+0", 0),
    ParseInt64Test::ok("1", 1),
    ParseInt64Test::ok("-1", -1),
    ParseInt64Test::ok("+1", 1),
    ParseInt64Test::ok("12345", 12345),
    ParseInt64Test::ok("-12345", -12345),
    ParseInt64Test::ok("012345", 12345),
    ParseInt64Test::ok("-012345", -12345),
    ParseInt64Test::ok("98765432100", 98765432100),
    ParseInt64Test::ok("-98765432100", -98765432100),
    ParseInt64Test::ok("9223372036854775807", i64::MAX),
    ParseInt64Test::ok("-9223372036854775807", -i64::MAX),
    ParseInt64Test::err_signed("9223372036854775808", i64::MAX, NumErrorCause::RangeSigned { bound_hint: i64::MAX }),
    ParseInt64Test::ok("-9223372036854775808", -1 << 63),
    ParseInt64Test::err_signed("9223372036854775809", 1<<63 - 1, NumErrorCause::RangeSigned { bound_hint: i64::MAX }),
    ParseInt64Test::err_signed("-9223372036854775809", -1 << 63, NumErrorCause::RangeSigned { bound_hint: i64::MIN }),
    ParseInt64Test::err_signed("-1_2_3_4_5", 0, NumErrorCause::Syntax), // base=10 so no underscores allowed
    ParseInt64Test::err_signed("-_12345", 0, NumErrorCause::Syntax),
    ParseInt64Test::err_signed("_12345", 0, NumErrorCause::Syntax),
    ParseInt64Test::err_signed("1__2345", 0, NumErrorCause::Syntax),
    ParseInt64Test::err_signed("12345_", 0, NumErrorCause::Syntax),
  ];

  static ref PARSE_INT64_BASE_TESTS: Vec<ParseInt64BaseTest> = vec![
    ParseInt64BaseTest::err_signed("", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::ok("0", 0, 0),
    ParseInt64BaseTest::ok("-0", 0, 0),
    ParseInt64BaseTest::ok("1", 0, 1),
    ParseInt64BaseTest::ok("-1", 0, -1),
    ParseInt64BaseTest::ok("12345", 0, 12345),
    ParseInt64BaseTest::ok("-12345", 0, -12345),
    ParseInt64BaseTest::ok("012345", 0, 0o12345),
    ParseInt64BaseTest::ok("-012345", 0, -0o12345),
    ParseInt64BaseTest::ok("0x12345", 0, 0x12345),
    ParseInt64BaseTest::ok("-0X12345", 0, -0x12345),
    ParseInt64BaseTest::err_signed("12345x", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("-12345x", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::ok("98765432100", 0, 98765432100),
    ParseInt64BaseTest::ok("-98765432100", 0, -98765432100),
    ParseInt64BaseTest::ok("9223372036854775807", 0, i64::MAX),
    ParseInt64BaseTest::ok("-9223372036854775807", 0, -i64::MAX),
    ParseInt64BaseTest::err_signed("9223372036854775808", 0, i64::MAX, NumErrorCause::RangeSigned{bound_hint:i64::MAX}),
    ParseInt64BaseTest::ok("-9223372036854775808", 0, i64::MIN),
    ParseInt64BaseTest::err_signed("9223372036854775809", 0, i64::MAX, NumErrorCause::RangeSigned{bound_hint:i64::MAX}),
    ParseInt64BaseTest::err_signed("-9223372036854775809", 0, i64::MIN, NumErrorCause::RangeSigned{bound_hint:i64::MIN}),

    // other bases
    ParseInt64BaseTest::ok("g", 17, 16),
    ParseInt64BaseTest::ok("10", 25, 25),
    ParseInt64BaseTest::ok("holycow", 35, (((((17*35+24)*35+21)*35+34)*35+12)*35+24)*35 + 32),
    ParseInt64BaseTest::ok("holycow", 36, (((((17*36+24)*36+21)*36+34)*36+12)*36+24)*36 + 32),

    // base 2
    ParseInt64BaseTest::ok("0", 2, 0),
    ParseInt64BaseTest::ok("-1", 2, -1),
    ParseInt64BaseTest::ok("1010", 2, 10),
    ParseInt64BaseTest::ok("1000000000000000", 2, 1 << 15),
    ParseInt64BaseTest::ok("111111111111111111111111111111111111111111111111111111111111111", 2, i64::MAX),
    ParseInt64BaseTest::err_signed("1000000000000000000000000000000000000000000000000000000000000000", 2, 1<<63 - 1, NumErrorCause::RangeSigned{bound_hint:i64::MAX}),
    ParseInt64BaseTest::ok("-1000000000000000000000000000000000000000000000000000000000000000", 2, -1 << 63),
    ParseInt64BaseTest::err_signed("-1000000000000000000000000000000000000000000000000000000000000001", 2, -1 << 63, NumErrorCause::RangeSigned{bound_hint:i64::MIN}),

    // base 8
    ParseInt64BaseTest::ok("-10", 8, -8),
    ParseInt64BaseTest::ok("57635436545", 8, 0o57635436545),
    ParseInt64BaseTest::ok("100000000", 8, 1 << 24),

    // base 16
    ParseInt64BaseTest::ok("10", 16, 16),
    ParseInt64BaseTest::ok("-123456789abcdef", 16, -0x123456789abcdef),
    ParseInt64BaseTest::ok("7fffffffffffffff", 16, i64::MAX),

    // underscores
    ParseInt64BaseTest::ok("-0x_1_2_3_4_5", 0, -0x12345),
    ParseInt64BaseTest::ok("0x_1_2_3_4_5", 0, 0x12345),
    ParseInt64BaseTest::err_signed("-_0x12345", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("_-0x12345", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("_0x12345", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("0x__12345", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("0x1__2345", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("0x1234__5", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("0x12345_", 0, 0, NumErrorCause::Syntax),

    ParseInt64BaseTest::ok("-0_1_2_3_4_5", 0, -0o12345), // octal
    ParseInt64BaseTest::ok("0_1_2_3_4_5", 0, 0o12345),   // octal
    ParseInt64BaseTest::err_signed("-_012345", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("_-012345", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("_012345", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("0__12345", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("01234__5", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("012345_", 0, 0, NumErrorCause::Syntax),

    ParseInt64BaseTest::ok("+0xf", 0, 0xf),
    ParseInt64BaseTest::ok("-0xf", 0, -0xf),
    ParseInt64BaseTest::err_signed("0x+f", 0, 0, NumErrorCause::Syntax),
    ParseInt64BaseTest::err_signed("0x-f", 0, 0, NumErrorCause::Syntax),
  ];

  static ref PARSE_UINT32_TESTS: Vec<ParseUint32Test> = vec![
      ParseUint32Test::err("", 0, NumErrorCause::Syntax),
      ParseUint32Test::ok("0", 0),
      ParseUint32Test::ok("1", 1),
      ParseUint32Test::ok("12345", 12345),
      ParseUint32Test::ok("012345", 12345),
      ParseUint32Test::err("12345x", 0, NumErrorCause::Syntax),
      ParseUint32Test::ok("987654321", 987654321),
      ParseUint32Test::ok("4294967295", u32::MAX),
      ParseUint32Test::err("4294967296", u32::MAX, NumErrorCause::RangeUnsigned {bound_hint: u32::MAX as u64}),
      ParseUint32Test::err("1_2_3_4_5", 0,NumErrorCause::Syntax), // base=10 so no underscores allowed
      ParseUint32Test::err("_12345", 0, NumErrorCause::Syntax),
      ParseUint32Test::err("_12345", 0, NumErrorCause::Syntax),
      ParseUint32Test::err("1__2345", 0, NumErrorCause::Syntax),
      ParseUint32Test::err("12345_", 0, NumErrorCause::Syntax),
  ];

  static ref PARSE_UINT64_BASE_TESTS: Vec<ParseUint64BaseTest> = vec![
    ParseUint64BaseTest::err("", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::ok("0", 0, 0),
    ParseUint64BaseTest::err("0x", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0X", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::ok("1", 0, 1),
    ParseUint64BaseTest::ok("12345", 0, 12345),
    ParseUint64BaseTest::ok("012345", 0, 0o12345),
    ParseUint64BaseTest::ok("0x12345", 0, 0x12345),
    ParseUint64BaseTest::ok("0X12345", 0, 0x12345),
    ParseUint64BaseTest::err("12345x", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0xabcdefg123", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("123456789abc", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::ok("98765432100", 0, 98765432100),
    ParseUint64BaseTest::ok("18446744073709551615", 0, u64::MAX),
    ParseUint64BaseTest::err("18446744073709551616", 0, u64::MAX, NumErrorCause::RangeUnsigned{bound_hint: 18446744073709551615}),
    ParseUint64BaseTest::err("18446744073709551620", 0, u64::MAX, NumErrorCause::RangeUnsigned{bound_hint: 18446744073709551615}),
    ParseUint64BaseTest::ok("0xFFFFFFFFFFFFFFFF", 0, u64::MAX),
    ParseUint64BaseTest::err("0x10000000000000000", 0, u64::MAX, NumErrorCause::RangeUnsigned{bound_hint: 18446744073709551615}),
    ParseUint64BaseTest::ok("01777777777777777777777", 0, u64::MAX),
    ParseUint64BaseTest::err("01777777777777777777778", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("02000000000000000000000", 0, u64::MAX, NumErrorCause::RangeUnsigned{bound_hint:  18446744073709551615}),
    ParseUint64BaseTest::ok("0200000000000000000000", 0, 1 << 61),
    ParseUint64BaseTest::err("0b", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0B", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::ok("0b101", 0, 5),
    ParseUint64BaseTest::ok("0B101", 0, 5),
    ParseUint64BaseTest::err("0o", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0O", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::ok("0o377", 0, 255),
    ParseUint64BaseTest::ok("0O377", 0, 255),

    // underscores allowed with base == 0 only
    ParseUint64BaseTest::ok("1_2_3_4_5", 0, 12345), // base 0 => 10
    ParseUint64BaseTest::err("_12345", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("1__2345", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("12345_", 0, 0, NumErrorCause::Syntax),

    ParseUint64BaseTest::err("1_2_3_4_5", 10, 0, NumErrorCause::Syntax), // base 10
    ParseUint64BaseTest::err("_12345", 10, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("1__2345", 10, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("12345_", 10, 0, NumErrorCause::Syntax),

    ParseUint64BaseTest::ok("0x_1_2_3_4_5", 0, 0x12345), // base 0 => 16
    ParseUint64BaseTest::err("_0x12345", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0x__12345", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0x1__2345", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0x1234__5", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0x12345_", 0, 0, NumErrorCause::Syntax),

    ParseUint64BaseTest::err("1_2_3_4_5", 16, 0, NumErrorCause::Syntax), // base 16
    ParseUint64BaseTest::err("_12345", 16, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("1__2345", 16, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("1234__5", 16, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("12345_", 16, 0, NumErrorCause::Syntax),

    ParseUint64BaseTest::ok("0_1_2_3_4_5", 0, 0o12345), // base 0 => 8 (0377)
    ParseUint64BaseTest::err("_012345", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0__12345", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("01234__5", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("012345_", 0, 0, NumErrorCause::Syntax),

    ParseUint64BaseTest::ok("0o_1_2_3_4_5", 0, 0o12345), // base 0 => 8 (0o377)
    ParseUint64BaseTest::err("_0o12345", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0o__12345", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0o1234__5", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0o12345_", 0, 0, NumErrorCause::Syntax),

    ParseUint64BaseTest::err("0_1_2_3_4_5", 8, 0, NumErrorCause::Syntax), // base 8
    ParseUint64BaseTest::err("_012345", 8, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0__12345", 8, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("01234__5", 8, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("012345_", 8, 0, NumErrorCause::Syntax),

    ParseUint64BaseTest::ok("0b_1_0_1", 0, 5), // base 0 => 2 (0b101)
    ParseUint64BaseTest::err("_0b101", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0b__101", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0b1__01", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0b10__1", 0, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("0b101_", 0, 0, NumErrorCause::Syntax),

    ParseUint64BaseTest::err("1_0_1", 2, 0, NumErrorCause::Syntax), // base 2
    ParseUint64BaseTest::err("_101", 2, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("1_01", 2, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("10_1", 2, 0, NumErrorCause::Syntax),
    ParseUint64BaseTest::err("101_", 2, 0, NumErrorCause::Syntax),
  ];

  static ref PARSE_UINT64_TESTS: Vec<ParseUint64Test> = vec![
    ParseUint64Test::err ("", 0, NumErrorCause::Syntax),
    ParseUint64Test::ok("0", 0),
    ParseUint64Test::ok("1", 1),
    ParseUint64Test::ok("12345", 12345),
    ParseUint64Test::ok("012345", 12345),
    ParseUint64Test::err("12345x", 0, NumErrorCause::Syntax),
    ParseUint64Test::ok("98765432100", 98765432100),
    ParseUint64Test::ok("18446744073709551615", u64::MAX),
    ParseUint64Test::err("18446744073709551616", u64::MAX, NumErrorCause::RangeUnsigned { bound_hint: u64::MAX }),
    ParseUint64Test::err("18446744073709551620", u64::MAX, NumErrorCause::RangeUnsigned { bound_hint: u64::MAX }),
    ParseUint64Test::err("1_2_3_4_5", 0, NumErrorCause::Syntax), // base=10 so no underscores allowed
    ParseUint64Test::err("_12345", 0, NumErrorCause::Syntax),
    ParseUint64Test::err("1__2345", 0, NumErrorCause::Syntax),
    ParseUint64Test::err("12345_", 0, NumErrorCause::Syntax),
    ParseUint64Test::err("-0", 0, NumErrorCause::Syntax),
    ParseUint64Test::err("-1", 0, NumErrorCause::Syntax),
    ParseUint64Test::err("+1", 0, NumErrorCause::Syntax),
  ];
}

struct ParseBaseTest<T> {
    input: &'static str,
    base: u8,
    output: T,
    err: Option<NumError>,
}

type ParseInt64BaseTest = ParseBaseTest<i64>;

type ParseUint64BaseTest = ParseBaseTest<u64>;

struct ParseTest<T> {
    input: &'static str,
    output: T,
    err: Option<NumError>,
}

type ParseInt32Test = ParseTest<i32>;

type ParseInt64Test = ParseTest<i64>;

type ParseUint32Test = ParseTest<u32>;

type ParseUint64Test = ParseTest<u64>;

impl<T> ParseBaseTest<T> {
    fn err(input: &'static str, base: u8, output: T, err: NumErrorCause) -> Self {
        let err = NumError {
            func: "parse_uint".to_string(),
            num: input.to_string(),
            err,
        };

        Self {
            input,
            base,
            output,
            err: Some(err),
        }
    }

    fn err_signed(input: &'static str, base: u8, output: T, err: NumErrorCause) -> Self {
        let err = NumError {
            func: "parse_int".to_string(),
            num: input.to_string(),
            err,
        };

        Self {
            input,
            base,
            output,
            err: Some(err),
        }
    }

    fn ok(input: &'static str, base: u8, output: T) -> Self {
        Self {
            input,
            base,
            output,
            err: None,
        }
    }
}

impl<T> ParseTest<T> {
    fn err_signed(input: &'static str, output: T, err: NumErrorCause) -> Self {
        let err = NumError {
            func: "parse_int".to_string(),
            num: input.to_string(),
            err,
        };

        Self {
            input,
            output,
            err: Some(err),
        }
    }

    fn ok(input: &'static str, output: T) -> Self {
        Self {
            input,
            output,
            err: None,
        }
    }

    fn err(input: &'static str, output: T, err: NumErrorCause) -> Self {
        let err = NumError {
            func: "parse_uint".to_string(),
            num: input.to_string(),
            err,
        };

        Self {
            input,
            output,
            err: Some(err),
        }
    }
}

#[test]
fn parse_int32() {
    for c in PARSE_INT32_TESTS.iter() {
        match strconv::parse_int(c.input, 10, 32) {
            Ok(got) => {
                assert!(c.err.is_none(), "unexpected error");
                assert_eq!(
                    got, c.output as i64,
                    "bad output for parse_int({}, 10, 32)",
                    c.input
                );
            }
            Err(err) => {
                let expect = c.err.as_ref().expect(&format!("miss error: {}", c.input));
                assert_eq!(&err, expect, "bad error for parse_int({}, 10, 32)", c.input);
            }
        }
    }
}

#[test]
fn parse_int64() {
    for c in PARSE_INT64_TESTS.iter() {
        match strconv::parse_int(c.input, 10, 64) {
            Ok(got) => {
                assert!(c.err.is_none(), "unexpected error");
                assert_eq!(
                    got, c.output as i64,
                    "bad output for parse_int({}, 10, 64)",
                    c.input
                );
            }
            Err(err) => {
                let expect = c.err.as_ref().expect(&format!("miss error: {}", c.input));
                assert_eq!(&err, expect, "bad error for parse_int({}, 10, 64)", c.input);
            }
        }
    }
}

#[test]
fn parse_int64_base() {
    for c in PARSE_INT64_BASE_TESTS.iter() {
        match strconv::parse_int(c.input, c.base, 64) {
            Ok(got) => {
                assert!(c.err.is_none(), "unexpected error");
                assert_eq!(
                    got, c.output,
                    "bad output for parse_int({}, {}, 64)",
                    c.input, c.base
                );
            }
            Err(err) => {
                //let expect = c.err.as_ref().expect(&format!("miss error: {}", c.input));
                assert_eq!(
                    Some(err),
                    c.err,
                    "bad error for parse_int({}, {}, 64)",
                    c.input,
                    c.base
                );
            }
        }
    }
}

#[test]
fn parse_uint32() {
    for c in PARSE_UINT32_TESTS.iter() {
        match strconv::parse_uint(c.input, 10, 32) {
            Ok(got) => {
                assert!(c.err.is_none(), "unexpected error");
                assert_eq!(
                    got, c.output as u64,
                    "bad output for parse_uint({}, 10, 32)",
                    c.input
                );
            }
            Err(err) => {
                let expect = c.err.as_ref().expect(&format!("miss error: {}", c.input));
                assert_eq!(&err, expect, "bad error");
            }
        }
    }
}

#[test]
fn parse_uint64() {
    for c in PARSE_UINT64_TESTS.iter() {
        match strconv::parse_uint(c.input, 10, 64) {
            Ok(got) => {
                assert!(c.err.is_none(), "unexpected error");
                assert_eq!(
                    got, c.output as u64,
                    "bad output for parse_uint({}, 10, 64)",
                    c.input
                );
            }
            Err(err) => {
                let expect = c.err.as_ref().expect(&format!("miss error: {}", c.input));
                assert_eq!(&err, expect, "bad error");
            }
        }
    }
}

#[test]
fn parse_uint64_base() {
    for c in PARSE_UINT64_BASE_TESTS.iter() {
        match strconv::parse_uint(c.input, c.base, 64) {
            Ok(got) => {
                assert!(c.err.is_none(), "unexpected error");
                assert_eq!(
                    got, c.output as u64,
                    "bad output for parse_uint({}, {}, 64)",
                    c.input, c.base
                );
            }
            Err(err) => {
                let expect = c.err.as_ref().expect(&format!("miss error: {}", c.input));
                assert_eq!(&err, expect, "bad error");
            }
        }
    }
}
