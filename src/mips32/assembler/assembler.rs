use super::parser::{Stmt, Stmt_, Expr, Expr_};
use super::lexer::Directive;
use super::register_name_to_number;

use crate::errors::{Span, MimicError, MimicErrorType};

use codespan_reporting::files::SimpleFile;

use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Instruction {
    span: Span,
    source: SimpleFile<String, String>,
    label: Option<String>,
    mnemonic: Expr,
    args: Vec<Expr>,
    inst: u32,
}

impl Instruction {
    pub fn builder(span: Span, mnemonic: String) -> InstructionBuilder {
        InstructionBuilder {
            span,
            mnemonic: Expr {
                span,
                node: Expr_::Ident(mnemonic)
            },
            args: vec![],
            label: None,
        }
    }

    fn unimplemented_instruction(&self) -> Result<(), MimicError> {
        Err(MimicError {
            span: Some(self.span),
            source: Some(self.source.clone()),
            ty: MimicErrorType::UnimplementedInstruction {
                mnemonic: self.get_mnemonic()
            }
        })
    }



    fn build_bytecode(&mut self, text_labels: &HashMap<String, u32>, location: usize) -> Result<(), MimicError> {
        let mnemonic = self.get_mnemonic();
        match mnemonic.as_str() {

            "add" => self.unimplemented_instruction()?,
            "addi" => {
                let (rt, rs, imm) = self.parse_itype1()?;
                let inst: u32 = (0x08 << 26) | (rs << 21) | (rt << 16) | imm;
                self.inst = inst;
            }
            "addiu" => {
                let (rt, rs, imm) = self.parse_itype1()?;
                let inst: u32 = (0x09 << 26) | (rs << 21) | (rt << 16) | imm;
                self.inst = inst;
            }
            "addu" => {
                let (rd, rs, rt) = self.parse_rtype()?;
                let inst: u32 = 0 | (rs << 21) | (rt << 16) | (rd << 11) | 0x21;
                self.inst = inst;
            }
            "and" => self.unimplemented_instruction()?,
            "andi" => {
                let (rt, rs, imm) = self.parse_itype1()?;
                let inst: u32 = (0x0C << 26) | (rs << 21) | (rt << 16) | imm;
                self.inst = inst;
            }
            "b" => self.unimplemented_instruction()?,
            "bal" => self.unimplemented_instruction()?,
            "beq" => {
                let (rs, rt, imm) = self.parse_branch(text_labels, location)?;
                let inst: u32 = (0x04 << 26) | (rs << 21) | (rt << 16) | (imm - 1);
                self.inst = inst;
            }
            "beql" => self.unimplemented_instruction()?,
            "bgez" => self.unimplemented_instruction()?,
            "bgezal" => self.unimplemented_instruction()?,
            "bgezall" => self.unimplemented_instruction()?,
            "bgezl" => self.unimplemented_instruction()?,
            "bgtz" => self.unimplemented_instruction()?,
            "bgtzl" => self.unimplemented_instruction()?,
            "blez" => self.unimplemented_instruction()?,
            "blezl" => self.unimplemented_instruction()?,
            "bltz" => self.unimplemented_instruction()?,
            "bltzal" => self.unimplemented_instruction()?,
            "bltzall" => self.unimplemented_instruction()?,
            "bltzl" => self.unimplemented_instruction()?,
            "bne" => {
                let (rs, rt, imm) = self.parse_branch(text_labels, location)?;
                let inst: u32 = (0x05 << 26) | (rs << 21) | (rt << 16) | (imm - 1);
                self.inst = inst;
            }
            "bnel" => self.unimplemented_instruction()?,
            "break" => self.unimplemented_instruction()?,
            "div" => self.unimplemented_instruction()?,
            "divu" => self.unimplemented_instruction()?,
            "j" => {
                let index = self.parse_jump(text_labels)?;
                let inst: u32 = (0x02 << 26) | (index + 0x00100000);
                self.inst = inst;
            }
            "jal" => self.unimplemented_instruction()?,
            "jalr" => self.unimplemented_instruction()?,
            "jr" => self.unimplemented_instruction()?,
            "lb" => self.unimplemented_instruction()?,
            "lbu" => self.unimplemented_instruction()?,
            "lh" => self.unimplemented_instruction()?,
            "lhu" => self.unimplemented_instruction()?,
            "ll" => self.unimplemented_instruction()?,
            "lui" => {
                let (rt, imm) = self.parse_itype2()?;
                let inst: u32 = (0x0F << 26) | (rt << 16) | imm;
                self.inst = inst;
            }
            "lw" => self.unimplemented_instruction()?,
            "madd" => self.unimplemented_instruction()?,
            "maddu" => self.unimplemented_instruction()?,
            "mfhi" => self.unimplemented_instruction()?,
            "mflo" => self.unimplemented_instruction()?,
            "msub" => self.unimplemented_instruction()?,
            "msubu" => self.unimplemented_instruction()?,
            "mthi" => self.unimplemented_instruction()?,
            "mtlo" => self.unimplemented_instruction()?,
            "mul" => self.unimplemented_instruction()?,
            "mult" => self.unimplemented_instruction()?,
            "nop" => self.unimplemented_instruction()?,
            "nor" => self.unimplemented_instruction()?,
            "or" => {
                let (rd, rs, rt) = self.parse_rtype()?;
                let inst: u32 = 0 | (rs << 21) | (rt << 16) | (rd << 11) | 0x25;
                self.inst = inst;
            }
            "ori" => {
                let (rt, rs, imm) = self.parse_itype1()?;
                let inst: u32 = (0x0D << 26) | (rs << 21) | (rt << 16) | imm;
                // println!("****{} - {:08x}", line, inst);
                self.inst = inst;
            }
            "sb" => self.unimplemented_instruction()?,
            "sh" => self.unimplemented_instruction()?,
            "sll" => {
                let (rd, rt, shmt) = self.parse_itype1()?;
                let inst: u32 = 0 | (rt << 16) | (rd << 11) | (shmt << 6);
                self.inst = inst;
            }
            "sllv" => self.unimplemented_instruction()?,
            "slt" => {
                let (rd, rs, rt) = self.parse_rtype()?;
                let inst: u32 = 0 | (rs << 21) | (rt << 16) | (rd << 11) | 0x2A;
                self.inst = inst;
            }
            "slti" => self.unimplemented_instruction()?,
            "sltiu" => self.unimplemented_instruction()?,
            "sltu" => self.unimplemented_instruction()?,
            "sra" => self.unimplemented_instruction()?,
            "srav" => self.unimplemented_instruction()?,
            "srl" => self.unimplemented_instruction()?,
            "srlv" => self.unimplemented_instruction()?,
            "sub" => self.unimplemented_instruction()?,
            "subu" => self.unimplemented_instruction()?,
            "sw" => self.unimplemented_instruction()?,
            "swl" => self.unimplemented_instruction()?,
            "swr" => self.unimplemented_instruction()?,
            "syscall" => {
                self.inst = 0x0000000C;
            }
            "xor" => {
                let (rd, rs, rt) = self.parse_rtype()?;
                let inst: u32 = 0 | (rs << 21) | (rt << 16) | (rd << 11) | 0x26;
                self.inst = inst;
            }
            "xori" => self.unimplemented_instruction()?,


            _ => return Err(MimicError {
                span: Some(self.mnemonic.span),
                source: Some(self.source.clone()),
                ty: MimicErrorType::UnknownMnemonic { mnemonic }
            })
        }
        Ok(())
    }

    fn parse_branch(&self, text_labels: &HashMap<String, u32>, location: usize) -> Result<(u32, u32, u32), MimicError> {
        let rs = self.get_register_arg(0)?;
        let rt = self.get_register_arg(1)?;
        let label = self.get_ident_arg(2)?;

        let offset = (text_labels[&label] as i32 - location as i32) as u32 & 0x0000FFFF;
        
        Ok((rs, rt, offset))
    }

    fn parse_jump(&self, text_labels: &HashMap<String, u32>) -> Result<u32, MimicError> {
        let label = self.get_ident_arg(0)?;
        Ok(text_labels[&label] as u32)
    }

    fn parse_rtype(&self) -> Result<(u32, u32, u32), MimicError> {
       let rd = self.get_register_arg(0)?;
       let rs = self.get_register_arg(1)?;
       let rt = self.get_register_arg(2)?;

       Ok((rd, rs, rt))
    }

    fn parse_itype1(&self) -> Result<(u32, u32, u32), MimicError> {
        let rt = self.get_register_arg(0)?;
        let rs = self.get_register_arg(1)?;
        let imm = self.get_integer_arg(2)?;

        Ok((rt, rs, imm))
    }

    fn parse_itype2(&self) -> Result<(u32, u32), MimicError> {
        let rt = self.get_register_arg(0)?;
        let imm = self.get_integer_arg(1)?;

        Ok((rt, imm))
    }

    fn get_mnemonic(&self) -> String {
        if let Expr_::Ident(m) = &self.mnemonic.node {
            return m.to_owned();
        }
        
        panic!("Mnemonic has incorrect expr type");
    }
    
    fn get_register_arg(&self, index: usize) -> Result<u32, MimicError> {
        return if let Some(arg) = self.args.get(index) {
            if let Expr_::Register(s) = &arg.node {
                register_name_to_number(s.to_owned(), &self.source).map(|v| v as u32)
            } else {
                Err(MimicError {
                    span: Some(arg.span),
                    source: Some(self.source.clone()),
                    ty: MimicErrorType::IncorrectArgumentType {},
                })
            }
        } else {
            Err(MimicError {
               span: Some(self.span),
                source: Some(self.source.clone()),
               ty: MimicErrorType::IncorrectArgument {}
            })
        }
    }

    fn get_integer_arg(&self, index: usize) -> Result<u32, MimicError> {
        return if let Some(arg) = self.args.get(index) {
            if let Expr_::IntLiteral(i) = &arg.node {
                // println!("get_integer_arg i: {}", i);
                Ok(*i as u32)
            } else {
                Err(MimicError {
                    span: Some(arg.span),
                    source: Some(self.source.clone()),
                    ty: MimicErrorType::IncorrectArgumentType {},
                })
            }
        } else {
            Err(MimicError {
                span: Some(self.span),
                source: Some(self.source.clone()),
                ty: MimicErrorType::IncorrectArgument {}
            })
        }
    }

    fn get_ident_arg(&self, index: usize) -> Result<String, MimicError> {
        return if let Some(arg) = self.args.get(index) {
            if let Expr_::Ident(s) = &arg.node {
                Ok(s.to_owned())
            } else {
                Err(MimicError {
                    span: Some(arg.span),
                    source: Some(self.source.clone()),
                    ty: MimicErrorType::IncorrectArgumentType {},
                })
            }
        } else {
            Err(MimicError {
                span: Some(self.span),
                source: Some(self.source.clone()),
                ty: MimicErrorType::IncorrectArgument {}
            })
        }
    }
}

struct InstructionBuilder {
    span: Span,
    mnemonic: Expr,
    args: Vec<Option<Expr>>,
    label: Option<String>
}

impl InstructionBuilder {
    pub fn build(&self, source: &SimpleFile<String, String>) -> Instruction {
        Instruction {
            span: self.span,
            source: source.clone(),
            label: self.label.clone(),
            mnemonic: self.mnemonic.clone(),
            args: self.args.iter()
                .map(|e| match e {
                    Some(x) => x.clone(),
                    None => panic!("Unfilled arguments in instruction builder"),
                })
                .collect(),
            inst: 0,
        }
    }

    // pub fn with_label(&mut self, label: String) -> &mut Self {
    //     self.label = Some(label);
    //     self
    // }

    pub fn with_opt_label(&mut self, label: Option<String>) -> &mut Self {
        self.label = label;
        self
    }

    pub fn with_ident_argument(&mut self, index: usize, ident: String) -> &mut Self {
        while self.args.len() <= index {
            self.args.push(None);
        }
        self.args[index] = Some(
            Expr {
                span: self.span,
                node: Expr_::Ident(ident.to_owned()),
            },
        );
        self
    }

    pub fn with_register_argument(&mut self, index: usize, register: String) -> &mut Self {
        while self.args.len() <= index {
            self.args.push(None);
        }
        self.args[index] = Some(
            Expr {
                span: self.span,
                node: Expr_::Register(register.to_owned()),
            },
        );
        self
    }

    pub fn with_integer_argument(&mut self, index: usize, integer: u32) -> &mut Self {
        while self.args.len() <= index {
            self.args.push(None);
        }
        self.args[index] = Some(
            Expr {
                span: self.span,
                node: Expr_::IntLiteral(integer as i64),
            },
        );
        self
    }
}
    

#[allow(unused_assignments)]
pub fn pack_data(data_section: Vec<Stmt>) -> (Vec<u8>, HashMap<String, u32>) {
    let mut data_bytes: Vec<u8> = Vec::new();
    let mut labels: HashMap<String, u32> = HashMap::new();

    let mut cur_index = 0;


    for stmt in data_section {

        if let Stmt_::DataDeclaration { label, type_directive, data } = stmt.statement {
            let mut label_str = String::new();


            if let Expr_::Label(x) = label.node {
                if let Expr_::Ident(s) = (*x).node {
                    label_str = s;
                } else {panic!("Unknown Error")}
            } else {panic!("Unknown Error")}

            if let Expr_::TypeDirectiveExpression(d) = type_directive.node {
                match d {
                    Directive::Asciiz => {
                        if data.len() > 1 {panic!(".asciiz data type does not support arrays")}
                        if data.len() == 0 {panic!(".asciiz requires a string")}
                        if let Expr_::StringLiteral(s) = &data[0].node {
                            // TODO replace other escaped characters
                            let mut val = s.replace("\\n", "\n");
                            val = val.strip_prefix("\"").unwrap().to_owned();
                            val = val.strip_suffix("\"").unwrap().to_owned();
                            for byte in val.as_bytes() {
                                data_bytes.push(*byte);
                            }
                            data_bytes.push(0x00);
                            // TODO: generalize data starting address
                            labels.insert(label_str, 0x10010000 + cur_index);
                            cur_index = data_bytes.len() as u32;
                            continue;

                        } else {panic!("Incorrect data type of .asciiz")}
                    },

                    _ => todo!(),
                }
            } else {panic!("Incorrect type directive")}



        } else {
            panic!("Incorrect statement in data section");
        }
    }


    (data_bytes, labels)
}

#[allow(unused_assignments)]
fn expand_instructions(text_section: Vec<Stmt>, data_labels: &HashMap<String, u32>, source: &SimpleFile<String, String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut cur_label: Option<String> = None;


    for inst_stmt in text_section {
        let span = inst_stmt.span;

        if let Stmt_::Instruction { mnemonic, args } = &inst_stmt.statement {


            if let Expr_::Ident(s) = &mnemonic.node {
                match s.as_str() {
                    "li" => {
                        let mut dest = "".to_owned();
                        let mut val = 0;

                        if let Expr_::Register(s) = &args[0].node {
                            dest = s.to_owned();    
                        } else {
                            panic!("Expected register");
                        }

                        if let Expr_::IntLiteral(i) = args[1].node {
                            val = i as u32;
                        } else {
                            panic!("Expected number");
                        }

                        if val > 0xFFFF {
                            instructions.push(
                                Instruction::builder(span, "lui".to_owned())
                                    .with_opt_label(cur_label)
                                    .with_register_argument(0, "$at".to_owned())
                                    .with_integer_argument(1, val >> 16)
                                    .build(source)
                            );
                            cur_label = None;
                            instructions.push(
                                Instruction::builder(span, "ori".to_owned())
                                    .with_register_argument(0, dest)
                                    .with_register_argument(1, "$at".to_owned())
                                    .with_integer_argument(2, val & 0x0000FFFF)
                                    .build(source)
                            );
                        } else {
                            
                            instructions.push(
                                Instruction::builder(span, "addiu".to_owned())
                                .with_opt_label(cur_label)
                                .with_register_argument(0, dest)
                                .with_register_argument(1, "$zero".to_owned())
                                .with_integer_argument(2, val)
                                .build(source)
                            );
                            cur_label = None;
                        }
                    }, // li

                    "la" => {
                        let mut dest = "".to_owned();
                        let mut label = "".to_owned();
                        let mut addr = 0;

                        if let Expr_::Register(s) = &args[0].node {
                            dest = s.to_owned();
                        } else {
                            panic!("Expected register");
                        }

                        if let Expr_::Ident(s) = &args[1].node {
                            label = s.to_owned();
                        } else {
                            panic!("Expected identifier");
                        }

                        if let Some(a) = data_labels.get(&label) {
                            addr = *a - 0x10010000;
                        } else {
                            panic!("Unknown label");
                        }

                        // println!("dest: {}", register_name_to_number(dest.clone()).unwrap());

                        instructions.push(
                            Instruction::builder(span, "lui".to_owned())
                            .with_opt_label(cur_label)
                            .with_register_argument(0, "$at".to_owned())
                            .with_integer_argument(1, 0x1001)
                            .build(source)
                        );
                        cur_label = None;
                        instructions.push(
                            Instruction::builder(span, "ori".to_owned())
                            .with_register_argument(0, dest)
                            .with_register_argument(1, "$at".to_owned())
                            .with_integer_argument(2, addr)
                            .build(source)
                        );

                        // println!("{:#010X?}", instructions[instructions.len() - 1]);
                    }, // la
                    
                    "move" => {
                        let mut dest = "".to_owned();
                        let mut src = "".to_owned();

                        if let Expr_::Register(s) = &args[0].node {
                            dest = s.to_owned();    
                        } else {
                            panic!("Expected register");
                        }

                        if let Expr_::Register(s) = &args[1].node {
                            src = s.to_owned();    
                        } else {
                            panic!("Expected register");
                        }

                        instructions.push(
                            Instruction::builder(span, "addu".to_owned())
                            .with_opt_label(cur_label)
                            .with_register_argument(0, dest)
                            .with_register_argument(1, "$zero".to_owned())
                            .with_register_argument(2, src)
                            .build(source)
                        );
                        cur_label = None;
                    }, // move

                    "blt" => {
                        let mut cmp1 = "".to_owned();
                        let mut cmp2 = "".to_owned();
                        let mut label = "".to_owned();

                        if let Expr_::Register(s) = &args[0].node {
                            cmp1 = s.to_owned();    
                        } else {
                            panic!("Expected register");
                        }

                        if let Expr_::Register(s) = &args[1].node {
                            cmp2 = s.to_owned();    
                        } else {
                            panic!("Expected register");
                        }

                        if let Expr_::Ident(s) = &args[2].node {
                            label = s.to_owned();
                        } else {
                            panic!("Expected identifier");
                        }

                        instructions.push(
                            Instruction::builder(span, "slt".to_owned())
                            .with_opt_label(cur_label)
                            .with_register_argument(0, "$at".to_owned())
                            .with_register_argument(1, cmp1)
                            .with_register_argument(2, cmp2)
                            .build(source)
                        );
                        cur_label = None;
                        instructions.push(
                            Instruction::builder(span, "bne".to_owned())
                            .with_register_argument(0, "$at".to_owned())
                            .with_register_argument(1, "$zero".to_owned())
                            .with_ident_argument(2, label)
                            .build(source)
                        );

                    }, // blt
                    _ => {
                        instructions.push(Instruction {
                            label: cur_label,
                            span,
                            source: source.clone(),
                            mnemonic: mnemonic.clone(),
                            args: args.to_vec(),
                            inst: 0,
                        });

                        cur_label = None;
                    }
                }

            } else {
                panic!("Expected mnemonic");
            }


        } else if let Stmt_::LabelDeclaration { label } = &inst_stmt.statement {
            if let Expr_::Label(l) = &label.node {
                if let Expr_::Ident(s) = &l.node {
                    cur_label = Some(s.to_owned());
                } else {
                    panic!("Expression other than Ident in LabelDeclaration statement");
                }
            } else {
                panic!("Expression other than Ident in LabelDeclaration statement");
            }
        } else {
            panic!("Non-instruction statement in .text");
        }

        // println!("{:#?}", inst_stmt);
    }
    

    instructions
}


pub fn assemble_ast(ast: Vec<Stmt>, source: &SimpleFile<String, String>) -> Result<(Vec<u8>, Vec<u8>), MimicError> {
    let mut data: Vec<Stmt> = Vec::new();
    let mut text: Vec<Stmt> = Vec::new();

    for section in ast {
        match section.statement {
            Stmt_::Section {section_directive, stmts} => {
                if let Expr_::SectionDirectiveExpression(d) = section_directive.node {
                    match d {
                        // Directive::Text => text = assemble_text(*stmts),
                        // Directive::Data => (data_bytes, data_labels) = pack_data(*stmts),
                        Directive::Data => data = *stmts,
                        Directive::Text => text = *stmts,
                        _ => todo!(),
                    }
                }
            }
            _ => panic!("Shouldn't be here..."),
        }
    }

    let (data_bytes, data_labels) = pack_data(data);

    let mut text_labels: HashMap<String, u32> = HashMap::new();
    let mut text_bytes: Vec<u8> = Vec::new();

    let mut instructions: Vec<Instruction> = expand_instructions(text, &data_labels, source);
    for (i, instruction) in instructions.iter().enumerate() {
        if let Some(label) = &instruction.label {
            text_labels.insert(label.to_owned(), i as u32);
        }
    }

    // println!("{:#08?}", text_labels);

    for (i, instruction) in instructions.iter_mut().enumerate() {
        instruction.build_bytecode(&text_labels, i)?;

        let inst = instruction.inst;

        // println!("{:#010X}", (inst >> 0) as u8);
        
        text_bytes.push((inst >> 0) as u8);
        text_bytes.push((inst >> 8) as u8);
        text_bytes.push((inst >> 16) as u8);
        text_bytes.push((inst >> 24) as u8);

    }


    // println!("{:#?}", text_bytes);

    // let text_bytes = assemble_text(text);

    // for byte in &data_bytes {
    //     println!("{}", *byte as char);
    // }


    // println!("{:#08X?}", data_labels);


    Ok((text_bytes, data_bytes))
    
}
