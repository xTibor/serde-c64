use std::collections::HashMap;
use std::fs::File;
use std::time::Duration;

use basic::PetsciiEncodingOptions;
use serde::Serialize;
use serde_c64::ContainerPrefixOptions;

#[derive(Serialize)]
struct Person {
    first_name: &'static str,
    last_name: &'static str,
    birth_year: isize,
}

#[derive(Serialize)]
enum CardSuit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

#[allow(clippy::type_complexity)]
#[derive(Serialize)]
struct TestStruct {
    people: Vec<Person>,
    route: Vec<(&'static str, (f32, f32))>,
    stand_cries: HashMap<&'static str, &'static str>,
    cards: [(usize, CardSuit); 5],
    song: Vec<(&'static str, usize, usize)>,
    card_colors: Vec<Option<(&'static str, CardSuit)>>,
    units: (((), ((), ())), ((), ()), u8),
    to_be_continued: Duration,
    string_escape: Vec<&'static str>,
}

fn main() {
    #[rustfmt::skip]
    let test_data = TestStruct {
        people: vec![
            Person { first_name: "Jonathan", last_name: "Joestar",     birth_year: 1868 },
            Person { first_name: "Joseph",   last_name: "Joestar",     birth_year: 1920 },
            Person { first_name: "Jotaro",   last_name: "Kujo",        birth_year: 1971 },
            Person { first_name: "Josuke",   last_name: "Higashikata", birth_year: 1983 },
            Person { first_name: "Giorno",   last_name: "Giovanna",    birth_year: 1985 },
            Person { first_name: "Jolyne",   last_name: "Cujoh",       birth_year: 1992 },
        ],
        route: vec![
            ("Tokyo",     (35.7642, 140.3849)),
            ("Hong Kong", (22.2948, 114.1661)),
            ("Singapore", ( 1.2804, 103.8441)),
            ("Calcutta",  (22.5432,  88.3662)),
            ("Varanasi",  (25.3127,  82.9855)),
            ("Karachi",   (24.8455,  66.9922)),
            ("Yabrin",    (23.3005,  48.9666)),
            ("Aswan",     (24.0889,  32.8986)),
            ("Kom Ombo",  (24.4770,  32.9457)),
            ("Luxor",     (25.6965,  32.6443)),
            ("Cairo",     (30.0746,  31.2450)),
        ],
        stand_cries: HashMap::from([
            ("Dio Brando",            "MUDAMUDAMUDA"),
            ("Giorno Giovanna",       "MUDAMUDAMUDA"),
            ("Jean Pierre Polnareff", "HORAHORAHORA"),
            ("Jolyne Cujoh",          "ORAORAORA"   ),
            ("Josuke Higashikata",    "DORARARA"    ),
            ("Jotaro Kujo",           "ORAORAORA"   ),
        ]),
        cards: [
            ( 8, CardSuit::Diamonds),
            ( 6, CardSuit::Spades  ),
            (10, CardSuit::Hearts  ),
            ( 1, CardSuit::Hearts  ),
            (11, CardSuit::Clubs   ),
        ],
        song: vec![
            ("f#", 5,  6), ("d",  5,  7), ("d",  5,  1), ("e",  5,  1),
            ("f",  5,  3), ("e",  5,  3), ("d",  5,  2), ("c#", 5,  3),
            ("d",  5,  3), ("e",  5,  2), ("f#", 5,  6), ("b",  5,  6),
            ("b",  4,  2), ("c#", 5,  2), ("d",  5,  3), ("e",  5,  3),
            ("d",  5,  2),
        ],
        card_colors: vec![
            Some(("red",   CardSuit::Diamonds)),
            Some(("black", CardSuit::Clubs   )),
            Some(("red",   CardSuit::Hearts  )),
            Some(("black", CardSuit::Spades  )),
            None,
        ],
        units: (((), ((), ())), ((), ()), 1),
        to_be_continued: Duration::from_secs(603300),
        string_escape: vec![
            "", "\"", "\"\"", "\"\"\"", "a\"", "\"a", "a\"a", "\"a\"", "A\"", "\"A", "A\"A", "\"A\"",
            "aa", "aA", "AA", "\"aa", "\"aA", "\"AA", "aa\"", "aA\"", "AA\"",
            " \"aa", " \"aA", " \"AA", " aa\"", " aA\"", " AA\"",
            "\"aa ", "\"aA ", "\"AA ", "aa\" ", "aA\" ", "AA\" ",
            " aa", " aA", " AA", "aa ", "aA ", "AA "," aa ", " aA ", " AA ",
            ",", "a,", ",a", "a,a", ",a,", "A,", ",A", "A,A", ",A,",
            " ,", " a,", " ,a", " a,a", " ,a,", " A,", " ,A", " A,A", " ,A,",
            ", ", "a, ", ",a ", "a,a ", ",a, ",  "A, ", ",A ", "A,A ", ",A, ",
            " , ", " a, ", " ,a ", " a,a ", " ,a, ", " A, ", " ,A ", " A,A ", " ,A, ",
        ],
    };

    let test_output = File::create("disk/test-output").unwrap();

    let options = serde_c64::Options {
        line_length: 64,
        line_number_start: 1000,
        line_number_increment: 1,
        encoding_options: PetsciiEncodingOptions {
            variant: basic::PetsciiVariant::Shifted,
        },
        container_prefix_options: ContainerPrefixOptions {
            sequence_length: true,
            map_length: true,
            tuple_length: false,
        },
        emit_enum_names: true,
    };

    serde_c64::to_writer(test_output, &test_data, options).unwrap();
}
