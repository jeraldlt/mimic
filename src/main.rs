use mimic_emulator::mips32::assembler::assemble_from_file;

use std::fs;

pub fn main() {

    let (text_bytes, _data_bytes) = assemble_from_file("test_files/mips32/bouncy.asm").unwrap();

    let text_bytes_correct = fs::read("test_files/mips32/bouncy.text").unwrap();
    let _data_bytes_correct = fs::read("test_files/mips32/bouncy.data").unwrap();


    let text_words = bytes_to_words(text_bytes);
    let text_words_correct = bytes_to_words(text_bytes_correct);

    let text_size = text_words.len().max(text_words_correct.len());
    for i in 0..text_size {
        let correct_word = text_words_correct.get(i).unwrap_or(&0);
        let word = text_words.get(i).unwrap_or(&0);
        
        if word == correct_word {
            println!("{:#010X} - {:#010X}", correct_word, word);
        } else {
            println!("{:#010X} - {:#010X}  !", correct_word, word);
        }
    }
    
}

fn bytes_to_words(bytes: Vec<u8>) -> Vec<u32> {
    let mut words: Vec<u32> = Vec::new();

    for i in 0..(bytes.len() / 4) {
        let mut inst: u32 = 0;
        inst |= (*bytes.get(i * 4 + 0).unwrap_or(&0) as u32) << 0;
        inst |= (*bytes.get(i * 4 + 1).unwrap_or(&0) as u32) << 8;
        inst |= (*bytes.get(i * 4 + 2).unwrap_or(&0) as u32) << 16;
        inst |= (*bytes.get(i * 4 + 3).unwrap_or(&0) as u32) << 24;

        words.push(inst);
   }

    words
}
