fn main() {
    const V32: &'static str = "-354634382";

    let got = strconv::parse_int(V32, 10, 32).unwrap();
    assert_eq!(got, -354634382i64);

    let _ = strconv::parse_int(V32, 16, 32).unwrap_err();

    const V64: &'static str = "-3546343826724305832";

    let got = strconv::parse_int(V64, 10, 64).unwrap();
    assert_eq!(got, -3546343826724305832);

    let _ = strconv::parse_int(V64, 16, 64).unwrap_err();
}
