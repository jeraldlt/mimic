use mimic_emulator::mips32::assembler::assemble_from_file;

use std::fs;

fn strip_trailing_zeros(input: Vec<u8>) -> Vec<u8> {
    let mut last_nonzero_index = input.len() - 1;

    while input[last_nonzero_index] == 0 {
        last_nonzero_index -= 1;
    }

    last_nonzero_index += 1;

    input[0..last_nonzero_index].to_owned()
}

#[test]
fn simple() {

    let (text_bytes, data_bytes) = assemble_from_file("test_files/mips32/simple.asm").unwrap();
    let data_bytes = strip_trailing_zeros(data_bytes);

    let text_bytes_correct = fs::read("test_files/mips32/simple.text").unwrap();
    let data_bytes_correct = strip_trailing_zeros(fs::read("test_files/mips32/simple.data").unwrap());

    assert_eq!(text_bytes, text_bytes_correct);
    assert_eq!(data_bytes, data_bytes_correct);
}

#[test]
fn bouncy() {

    let (text_bytes, data_bytes) = assemble_from_file("test_files/mips32/bouncy.asm").unwrap();
    let data_bytes = strip_trailing_zeros(data_bytes);

    let text_bytes_correct = fs::read("test_files/mips32/bouncy.text").unwrap();
    let data_bytes_correct = strip_trailing_zeros(fs::read("test_files/mips32/bouncy.data").unwrap());

    assert_eq!(text_bytes, text_bytes_correct);
    assert_eq!(data_bytes, data_bytes_correct);
}
