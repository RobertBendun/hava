use crate::raw::*;

#[derive(Debug)]
pub enum Instruction {
    ALoad(u8),
    GetStatic(u16),
    InvokeSpecial(u16),
    Return,
    Nop,
    Ldc(u8),
    InvokeVirtual(u16),
}

impl Instruction {
    pub fn from<F: ByteUtils>(file: &mut F) -> std::io::Result<Instruction> {
        let opcode = file.read_u1()?;
        Ok(match opcode {
            0x0 => Instruction::Nop,
            0xb1 => Instruction::Return,
            0x2a | 0x2b | 0x2c | 0x2d => Instruction::ALoad(opcode - 0x2a),
            0x12 => Instruction::Ldc(file.read_u1()?),
            0xb2 => {
                let indexbyte1 = file.read_u1()? as u16;
                let indexbyte2 = file.read_u1()? as u16;
                let index = (indexbyte1 << 8) | indexbyte2;
                Instruction::GetStatic(index)
            }
            0xb6 => {
                let indexbyte1 = file.read_u1()? as u16;
                let indexbyte2 = file.read_u1()? as u16;
                let index = (indexbyte1 << 8) | indexbyte2;
                Instruction::InvokeVirtual(index)
            }
            0xb7 => {
                let indexbyte1 = file.read_u1()? as u16;
                let indexbyte2 = file.read_u1()? as u16;
                let index = (indexbyte1 << 8) | indexbyte2;
                Instruction::InvokeSpecial(index)
            }
            _ => {
                println!("Unrecognized opcode: {opcode:#x}");
                panic!();
            }
        })
    }
}
