use rstest::rstest;
use string_comps::levenshtein_distance; // Adjust the import based on actual module structure

#[rstest]
#[case("", "", 0)]
#[case("", "\0", 1)]
#[case("", "abc", 3)]
#[case("abc", "", 3)]
#[case("sitting", "sitting", 0)]
#[case("sitting", "kitten", 3)]
#[case("example", "samples", 3)]
#[case("distance", "difference", 5)]
#[case("test", "text", 1)]
#[case("test", "tset", 2)]
#[case("test", "qwe", 4)]
#[case("test", "testit", 2)]
#[case("test", "tesst", 1)]
#[case("test", "tet", 1)]
#[case("gumbo", "gambol", 2)]
#[case("saturday", "sunday", 3)]
#[case("a", "b", 1)]
#[case("ab", "ac", 1)]
#[case("ac", "bc", 1)]
#[case("abc", "axc", 1)]
#[case("xabxcdxxefxgx", "1ab2cd34ef5g6", 6)]
#[case("xabxcdxxefxgx", "abcdefg", 6)]
#[case("javawasneat", "scalaisgreat", 7)]
#[case("sturgeon", "urgently", 6)]
#[case("levenshtein", "frankenstein", 6)]
#[case("distance", "difference", 5)]
#[case("kitten", "sitting", 3)]
#[case("Tier", "Tor", 2)]
#[case("Rust", "rust", 1)]
#[case("book", "bookworm", 4)]
fn test_levenshtein(#[case] s1: &str, #[case] s2: &str, #[case] expected: usize) {
    assert_eq!(
        levenshtein_distance(s1, s2),
        expected,
        "Failed on input: ({}, {})",
        s1,
        s2
    );
}
