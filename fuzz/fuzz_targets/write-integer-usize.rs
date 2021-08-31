#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
use lexical_util::constants::BUFFER_SIZE;
use lexical_write_integer::ToLexical;

fuzz_target!(|value: usize| {
    let mut buffer = [b'0'; BUFFER_SIZE];
    let _ = value.to_lexical(&mut buffer);
});
