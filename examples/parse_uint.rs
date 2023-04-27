fn main() {
    const V: &'static str = "42";

    let got = strconv::parse_uint(V, 10, 32);
    assert_eq!(got, Ok(42));

    let got = strconv::parse_uint(V, 10, 64);
    assert_eq!(got, Ok(42));
}
