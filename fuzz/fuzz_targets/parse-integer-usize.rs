#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
use lexical_parse_integer::FromLexical;

fuzz_target!(|data: &[u8]| {
    let _ = usize::from_lexical(data);
});
