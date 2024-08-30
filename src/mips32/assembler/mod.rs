pub mod lexer;
pub mod parser;
pub mod assembler;

use lexer::{Lexer, Token};
use parser::{parse, Stmt};
use assembler::assemble_ast;

use crate::errors::{MimicError, MimicErrorType};

use codespan_reporting::files::SimpleFile;

use std::path::Path;


pub fn assemble_from_string(contents: String) -> Result<(Vec<u8>, Vec<u8>), MimicError> {
    let file: SimpleFile<String, String> = SimpleFile::new("".to_owned(), contents);

    assemble(file)
}

pub fn assemble_from_file<P>(filename: P) -> Result<(Vec<u8>, Vec<u8>), MimicError>
where
    P: AsRef<Path>,
{
    let contents = match std::fs::read_to_string(&filename) {
        Ok(s) => s,
        Err(_) => return Err(MimicError {
            span: None,
            source: None,
            ty: MimicErrorType::FileDoesNotExist {filename: filename.as_ref().to_path_buf()},
        }),       
    };

    let file: SimpleFile<String, String> = SimpleFile::new(filename.as_ref().to_path_buf().file_name().unwrap().to_owned().into_string().unwrap(), contents.to_owned());

    assemble(file)
}

fn assemble(file: SimpleFile<String, String>) -> Result<(Vec<u8>, Vec<u8>), MimicError> {

    let tokens = Lexer::new(file.source().as_str(), file.clone());

    let mut valid = true;
    for (tok, _span) in tokens.clone() {
        match tok {
            Token::Unknown(_) => valid = false,
            _ => {},
        }

        // println!("{:?}", tok);
    }

    if !valid { 
        // bail
    }

    let ast: Vec<Stmt> = parse(tokens).unwrap();

    let (text_bytes, data_bytes) = assemble_ast(ast, &file)?;


    // let (data_bytes, data_labels) = pack_data(&data);
    // let (text_expanded, text_labels) = expand_pseudoinstructions(&text, &data_labels);
    // let text_bytes = assemble_instructions(&text_expanded, &text_labels)?;

    // for (i, byte) in (&data_bytes).iter().enumerate() {
    //     println!("{}: {}", i, *byte as char);
    // }

    // let mut file = File::create("test2.data").unwrap();
    // file.write_all(data_bytes.as_slice()).unwrap();

    Ok((text_bytes, data_bytes))
}


fn register_name_to_number(reg: String, source: &SimpleFile<String, String>) -> Result<usize, MimicError> {
    let reg = reg.strip_prefix("$").unwrap_or(&reg);

    match reg.to_lowercase().as_str() {
        "0" | "zero" => Ok(0),
        "1" | "at" => Ok(1),
        "2" | "v0" => Ok(2),
        "3" | "v1" => Ok(3),
        "4" | "a0" => Ok(4),
        "5" | "a1" => Ok(5),
        "6" | "a2" => Ok(6),
        "7" | "a3" => Ok(7),
        "8" | "t0" => Ok(8),
        "9" | "t1" => Ok(9),
        "10" | "t2" => Ok(10),
        "11" | "t3" => Ok(11),
        "12" | "t4" => Ok(12),
        "13" | "t5" => Ok(13),
        "14" | "t6" => Ok(14),
        "15" | "t7" => Ok(15),
        "16" | "s0" => Ok(16),
        "17" | "s1" => Ok(17),
        "18" | "s2" => Ok(18),
        "19" | "s3" => Ok(19),
        "20" | "s4" => Ok(20),
        "21" | "s5" => Ok(21),
        "22" | "s6" => Ok(22),
        "23" | "s7" => Ok(23),
        "24" | "t8" => Ok(24),
        "25" | "t9" => Ok(25),
        "26" | "k0" => Ok(26),
        "27" | "k1" => Ok(27),
        "28" | "gp" => Ok(28),
        "29" | "sp" => Ok(29),
        "30" | "fp" => Ok(30),
        "31" | "ra" => Ok(31),

        _ => Err(MimicError {
            span: None,
            source: Some(source.clone()),
            ty: MimicErrorType::UnknownRegister{register_name: reg.to_owned()}
        }),
    }
}

// fn immediate_str_to_u32(imm: &String) -> u32 {
//     return if imm.starts_with("0x") {
//         u32::from_str_radix(imm.strip_prefix("0x").unwrap(), 16).unwrap()
//     } else {
//         imm.parse::<u32>().unwrap()
//     };
// }
//
// // Parses instructions formated as "mnemonic $rd, $rs, $rt"
// fn parse_rtype(line: &String) -> Result<(u32, u32, u32), MimicError> {
//     let re = Regex::new(r"^\s*([a-zA-Z]+)\s*\$([a-zA-z]+[a-zA-Z0-9]*)\s*,?\s*\$([a-zA-Z0-9]*)\s*,?\s*\$([a-zA-Z_]+[a-zA-Z0-9_]*).*$").unwrap();
//
//     let caps = re
//         .captures(line.as_str())
//         .expect(format!("Error capturing {}", line).as_str());
//
//     let rd = register_name_to_number(caps.get(2).unwrap().as_str().to_owned())? as u32;
//     let rs = register_name_to_number(caps.get(3).unwrap().as_str().to_owned())? as u32;
//     let rt = register_name_to_number(caps.get(4).unwrap().as_str().to_owned())? as u32;
//
//     Ok((rd, rs, rt))
// }
//
// // Parses instructions formated as "mnemonic $rt, $rs, imm"
// fn parse_itype1(line: &String) -> Result<(u32, u32, u32), MimicError> {
//     let re = Regex::new(r"^\s*([a-zA-Z]+)\s*\$([a-zA-z]+[a-zA-Z0-9]*)\s*,?\s*\$([a-zA-Z0-9]*)\s*,?\s*([a-zA-Z0-9_]+).*$").unwrap();
//
//     let caps = re.captures(line.as_str()).unwrap();
//
//     let rt = register_name_to_number(caps.get(2).unwrap().as_str().to_owned())? as u32;
//     let rs = register_name_to_number(caps.get(3).unwrap().as_str().to_owned())? as u32;
//     // let imm = u32::from_str_radix(caps.get(4).unwrap().as_str().strip_prefix("0x").unwrap(), 16).unwrap();
//     let imm = immediate_str_to_u32(&caps.get(4).unwrap().as_str().to_owned());
//
//     Ok((rt, rs, imm))
// }
//
// // Parses instructions formated as "mnemonic $rt, imm"
// fn parse_itype2(line: &String) -> Result<(u32, u32), MimicError> {
//     let re = Regex::new(r"^\s*([a-zA-Z]+)\s*\$([a-zA-z]+[a-zA-Z0-9]*)\s*,?\s*([a-zA-Z0-9_]+).*$")
//         .unwrap();
//
//     let caps = re.captures(line.as_str()).unwrap();
//
//     let rt = register_name_to_number(caps.get(2).unwrap().as_str().to_owned())? as u32;
//     let imm = u32::from_str_radix(
//         caps.get(3).unwrap().as_str().strip_prefix("0x").unwrap(),
//         16,
//     )
//     .unwrap();
//
//     Ok((rt, imm))
// }
//
// // Parses instructions formated as "mnemonic $rt, $rs, imm"
// fn parse_branch(
//     line: &String,
//     text_labels: &HashMap<String, usize>,
//     pc_words: usize,
// ) -> Result<(u32, u32, u32), MimicError> {
//     let re = Regex::new(r"^\s*([a-zA-Z]+)\s*\$([a-zA-z]+[a-zA-Z0-9]*)\s*,?\s*\$([a-zA-Z0-9]*)\s*,?\s*([a-zA-Z0-9_]+).*$").unwrap();
//
//     let caps = re.captures(line.as_str()).unwrap();
//
//     let rs = register_name_to_number(caps.get(2).unwrap().as_str().to_owned())? as u32;
//     let rt = register_name_to_number(caps.get(3).unwrap().as_str().to_owned())? as u32;
//     let label = caps.get(4).unwrap().as_str().to_owned();
//
//     let offset = (text_labels[&label] as i32 - pc_words as i32) & 0x0000FFFF;
//
//     // println!("Branch offset for {}: {:#04X}", line, offset);
//
//     Ok((rs, rt, offset as u32))
// }
//
// fn parse_jump(line: &String, text_labels: &HashMap<String, usize>) -> Result<u32, MimicError> {
//     let re = Regex::new(r"^\s*([a-zA-Z]+)\s+([a-zA-Z_]+[a-zA-Z0-9_]*).*$").unwrap();
//     let caps = re.captures(line.as_str()).unwrap();
//     let label = caps.get(2).unwrap().as_str().to_owned();
//
//     Ok(text_labels[&label] as u32)
// }
//
// fn assemble_instructions(
//     text_expanded: &Vec<String>,
//     text_labels: &HashMap<String, usize>,
// ) -> Result<Vec<u8>, MimicError> {
//     let mut instructions: Vec<u32> = Vec::new();
//
//     let re = Regex::new(r"^\s*([a-zA-z_]+)\s*[^:]*$").unwrap();
//     for (i, line) in text_expanded.iter().enumerate() {
//         let caps = re.captures(line).unwrap();
//         let mnemonic = caps.get(1).unwrap().as_str();
//         match mnemonic {
//             "add" => {}
//             "addi" => {
//                 let (rt, rs, imm) = parse_itype1(line)?;
//                 let inst: u32 = (0x08 << 26) | (rs << 21) | (rt << 16) | imm;
//                 instructions.push(inst);
//             }
//             "addiu" => {
//                 let (rt, rs, imm) = parse_itype1(line)?;
//                 let inst: u32 = (0x09 << 26) | (rs << 21) | (rt << 16) | imm;
//                 instructions.push(inst);
//             }
//             "addu" => {
//                 let (rd, rs, rt) = parse_rtype(line)?;
//                 let inst: u32 = 0 | (rs << 21) | (rt << 16) | (rd << 11) | 0x21;
//                 instructions.push(inst);
//             }
//             "and" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "andi" => {
//                 let (rt, rs, imm) = parse_itype1(line)?;
//                 let inst: u32 = (0x0C << 26) | (rs << 21) | (rt << 16) | imm;
//                 instructions.push(inst);
//             }
//             "b" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bal" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "beq" => {
//                 let (rs, rt, imm) = parse_branch(line, text_labels, i)?;
//                 let inst: u32 = (0x04 << 26) | (rs << 21) | (rt << 16) | imm;
//                 instructions.push(inst);
//             }
//             "beql" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bgez" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bgezal" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bgezall" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bgezl" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bgtz" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bgtzl" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "blez" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "blezl" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bltz" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bltzal" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bltzall" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bltzl" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "bne" => {
//                 let (rs, rt, imm) = parse_branch(line, text_labels, i)?;
//                 let inst: u32 = (0x05 << 26) | (rs << 21) | (rt << 16) | imm;
//                 instructions.push(inst);
//             }
//             "bnel" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "break" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "div" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "divu" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "j" => {
//                 let index = parse_jump(line, text_labels)?;
//                 let inst: u32 = (0x02 << 26) | (index + 0x00100000);
//                 instructions.push(inst);
//             }
//             "jal" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "jalr" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "jr" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "lb" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "lbu" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "lh" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "lhu" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "ll" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "lui" => {
//                 let (rt, imm) = parse_itype2(line)?;
//                 let inst: u32 = (0x0F << 26) | (rt << 16) | imm;
//                 instructions.push(inst);
//             }
//             "lw" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "madd" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "maddu" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "mfhi" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "mflo" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "msub" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "msubu" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "mthi" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "mtlo" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "mul" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "mult" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "nop" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "nor" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "or" => {
//                 let (rd, rs, rt) = parse_rtype(line)?;
//                 let inst: u32 = 0 | (rs << 21) | (rt << 16) | (rd << 11) | 0x25;
//                 instructions.push(inst);
//             }
//             "ori" => {
//                 let (rt, rs, imm) = parse_itype1(line)?;
//                 let inst: u32 = (0x0D << 26) | (rs << 21) | (rt << 16) | imm;
//                 // println!("****{} - {:08x}", line, inst);
//                 instructions.push(inst);
//             }
//             "sb" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "sh" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "sll" => {
//                 let (rd, rt, shmt) = parse_itype1(line)?;
//                 let inst: u32 = 0 | (rt << 16) | (rd << 11) | (shmt << 6);
//                 instructions.push(inst);
//             }
//             "sllv" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "slt" => {
//                 let (rd, rs, rt) = parse_rtype(line)?;
//                 let inst: u32 = 0 | (rs << 21) | (rt << 16) | (rd << 11) | 0x2A;
//                 instructions.push(inst);
//             }
//             "slti" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "sltiu" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "sltu" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "sra" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "srav" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "srl" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "srlv" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "sub" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "subu" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "sw" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "swl" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "swr" => todo!("Instuction not yet implemented: {}", mnemonic),
//             "syscall" => {
//                 instructions.push(0x0000000C);
//             }
//             "xor" => {
//                 let (rd, rs, rt) = parse_rtype(line)?;
//                 let inst: u32 = 0 | (rs << 21) | (rt << 16) | (rd << 11) | 0x26;
//                 instructions.push(inst);
//             }
//             "xori" => todo!("Instuction not yet implemented: {}", mnemonic),
//
//             _ => {
//                 return Err(MimicError {
//                     span: None,
//                     ty: MimicErrorType::UnknownMnemonic{mnemonic: mnemonic.to_owned()}
//                 });
//             }
//         }
//     }
//
//     let mut bytes: Vec<u8> = Vec::new();
//
//     // for (i, inst) in instructions.iter().enumerate() {
//     //     println!("{:08x}", inst);
//     // }
//
//     for inst in instructions {
//         bytes.push((inst >> 0) as u8);
//         bytes.push((inst >> 8) as u8);
//         bytes.push((inst >> 16) as u8);
//         bytes.push((inst >> 24) as u8);
//     }
//
//     Ok(bytes)
// }
//
// fn expand_pseudoinstructions(
//     text_str: &Vec<String>,
//     data_labels: &HashMap<String, usize>,
// ) -> (Vec<String>, HashMap<String, usize>) {
//     let mut text_expanded: Vec<String> = Vec::new();
//     let mut text_labels: HashMap<String, usize> = HashMap::new();
//
//     let re = Regex::new(r"^\s*([a-zA-z_]+)\s*[^:]*$").unwrap();
//     for line in text_str {
//         if let Some(caps) = re.captures(line) {
//             match caps.get(1).unwrap().as_str() {
//                 "li" => {
//                     let re_li = Regex::new(
//                         r"^\s*([a-zA-z_]+)\s*\$([a-zA-z]+[0-9]?)\s*,?\s*([a-zA-z0-9]+).*$",
//                     )
//                     .unwrap();
//                     let caps_li = re_li.captures(line).unwrap();
//
//                     let val;
//                     let val_str = caps_li.get(3).unwrap().as_str();
//                     if val_str.starts_with("0x") {
//                         val = u32::from_str_radix(val_str.strip_prefix("0x").unwrap(), 16).unwrap();
//                     } else {
//                         val = val_str.parse::<u32>().unwrap();
//                     }
//                     let dest = caps_li.get(2).unwrap().as_str();
//
//                     if val > 0xFFFF {
//                         text_expanded.push(format!("lui $at, {:#0X}", val >> 16));
//                         text_expanded.push(format!("ori ${}, $at, {:#0X}", dest, val & 0x0000FFFF));
//                     } else {
//                         text_expanded.push(format!("addiu ${}, $zero, {:#0X}", dest, val));
//                     }
//                 }
//
//                 "lw" => todo!(),
//
//                 "move" => {
//                     let re_move = Regex::new(
//                         r"^\s*([a-zA-z_]+)\s*\$([a-zA-z]+[0-9]?)\s*,?\s*\$([a-zA-z]+[0-9]?).*$",
//                     )
//                     .unwrap();
//                     let caps_move = re_move.captures(line).unwrap();
//                     let src = caps_move.get(3).unwrap().as_str();
//                     let dest = caps_move.get(2).unwrap().as_str();
//                     text_expanded.push(format!("addu ${}, $zero, ${}", dest, src));
//                 }
//
//                 "la" => {
//                     let re_la = Regex::new(r"^\s*([a-zA-z_]+)\s*\$([a-zA-z]+[0-9]?)\s*,?\s*([a-zA-Z_]+[a-zA-z0-9_]*).*$").unwrap();
//                     let caps_la = re_la.captures(line).unwrap();
//                     let dest = caps_la.get(2).unwrap().as_str();
//                     let label = caps_la.get(3).unwrap().as_str();
//                     let addr = data_labels[label];
//
//                     text_expanded.push(format!("lui $at, {:#0X}", addr >> 16));
//                     text_expanded.push(format!("ori ${}, $at, {:#0X}", dest, addr & 0x0000FFFF));
//                 }
//
//                 "blt" => {
//                     let re_blt = Regex::new(r"^\s*([a-zA-Z]+)\s*(\$[a-zA-z]+[a-zA-Z0-9]*)\s*,?\s*(\$[a-zA-Z0-9]*)\s*,?\s*([a-zA-Z_]+[a-zA-Z0-9_]*).*$").unwrap();
//                     let caps_blt = re_blt.captures(line).unwrap();
//                     let cmp1 = caps_blt.get(2).unwrap().as_str();
//                     let cmp2 = caps_blt.get(3).unwrap().as_str();
//                     let label = caps_blt.get(4).unwrap().as_str();
//                     text_expanded.push(format!("slt $at, {}, {}", cmp1, cmp2));
//                     text_expanded.push(format!("bne $at, $zero, {}", label));
//                 }
//
//                 "ble" => todo!(),
//                 "bgt" => todo!(),
//                 "bge" => todo!(),
//
//                 _ => text_expanded.push(line.clone()),
//             }
//         } else {
//             let re_label = Regex::new(r"^\s*([a-zA-Z_]+[a-zA-z0-9_]*):.*$").unwrap();
//             if let Some(cap) = re_label.captures(line) {
//                 let label = cap.get(1).unwrap().as_str();
//                 text_labels.insert(label.to_owned(), text_expanded.len() - 1);
//             } else {
//                 text_expanded.push(format!("??? -> {}", line.clone()));
//             }
//         }
//     }
//
//     // let mut text_labeled: Vec<String> = Vec::new();
//
//     // for line in &text_expanded {
//     //     let mut added = false;
//     //     'outerloop: for (k, v) in text_labels.iter() {
//     //         if line.contains(k) {
//     //             text_labeled.push(line.replace(k, format!("{}", v).as_str()));
//     //             added = true;
//     //             break 'outerloop
//     //         }
//     //     }
//
//     //     if !added {
//     //         text_labeled.push(line.clone());
//     //     }
//     // }
//
//     // for line in &text_expanded {
//     // println!("{}", line);
//     // }
//
//     // println!("{:?}", &text_labels);
//
//     (text_expanded, text_labels)
// }
//
// fn pack_data(data_str: &Vec<String>) -> (Vec<u8>, HashMap<String, usize>) {
//     let mut data_bytes: Vec<u8> = Vec::new();
//     let mut labels: HashMap<String, usize> = HashMap::new();
//
//     let mut cur_index = 0;
//
//     let re = Regex::new(r"^\s*([a-zA-z_]+.*)\s*:\s*(.asciiz|.asciiz\[\]|.byte)\s+(.*)$").unwrap();
//     for line in data_str {
//         if let Some(caps) = re.captures(line) {
//             let label = caps.get(1).unwrap();
//             match caps.get(2).unwrap().as_str() {
//                 ".asciiz" => {
//                     let val = caps
//                         .get(3)
//                         .unwrap()
//                         .as_str()
//                         .strip_prefix("\"")
//                         .unwrap()
//                         .strip_suffix("\"")
//                         .unwrap()
//                         .to_owned();
//
//                     let val = val.replace("\\n", "\n");
//                     for byte in val.as_bytes() {
//                         data_bytes.push(*byte);
//                     }
//                     data_bytes.push(0x00);
//
//                     // let needed_size = 4 * ((data_bytes.len() + val.len()) / 4 + 1);
//                     // // println!("Needed size: {needed_size}, val.len() {}", val.len());
//                     // if data_bytes.capacity() < needed_size {
//                     //     data_bytes.reserve(needed_size - data_bytes.capacity());
//                     // }
//                     // while data_bytes.len() < needed_size {
//                     //     data_bytes.push(0x00);
//                     // }
//
//                     // for (k, byte) in val.as_bytes().iter().enumerate() {
//                     //     let i = 4 * (k / 4) + (3 - k % 4);
//                     //     // println!("{k}, {i}, {}", cur_index + i);
//                     //     data_bytes[cur_index + i] = *byte;
//                     // }
//
//                     // if (cur_index + val.len()) % 4 == 0 {
//                     //     data_bytes.push(0x00);
//                     // }
//
//                     // while data_bytes.len() % 4 != 0 {
//                     //     data_bytes.push(0x00);
//                     // }
//
//                     labels.insert(label.as_str().to_owned(), 0x10010000 + cur_index);
//
//                     cur_index = data_bytes.len();
//                 }
//
//                 _ => {}
//             }
//         }
//     }
//
//     // for (i, byte) in (&data_bytes).iter().enumerate() {
//     //     if i % 4 == 0 {
//     //         println!("");
//     //     }
//     //     println!("{i}: {:#04X}, {}", byte, *byte as char);
//     // }
//
//     // println!("{:?}", &labels);
//
//     // let mut data_words: Vec<u32> = Vec::new();
//
//     // for chunk in data_bytes.chunks(4) {
//     //     let mut word: u32 = (chunk[0] as u32) << 24;
//     //     word |= (chunk[1] as u32) << 16;
//     //     word |= (chunk[2] as u32) << 8;
//     //     word |= chunk[3] as u32;
//
//     //     data_words.push(word);
//     // }
//
//     (data_bytes, labels)
// }

//
// fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>, MimicError>
// where
//     P: AsRef<Path>,
// {
//     match File::open(&filename) {
//         Ok(f) => return Ok(io::BufReader::new(f).lines()),
//         Err(_) => {
//             return Err(MimicError {
//                 span: None,
//                 ty: MimicErrorType::FileDoesNotExist{filename: filename.as_ref().to_owned()}
//             })
//         }
//     }
// }
//
// fn strip_line(mut line: String) -> String {
//     line = line.trim().to_owned();
//     line = line.split('#').next().unwrap().to_owned();
//     line = line.trim().to_owned();
//
//     line
// }
//
//
