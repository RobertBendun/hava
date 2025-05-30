use crate::raw::*;

#[derive(Debug)]
pub enum Instruction {
    ALoad0,
    ALoad1,
    ALoad2,
    ALoad3,
    BiPush(u8),
    GetStatic(u16),
    IAdd,
    ILoad0,
    ILoad1,
    ILoad2,
    ILoad3,
    IStore0,
    IStore1,
    IStore2,
    IStore3,
    InvokeSpecial(u16),
    InvokeVirtual(u16),
    Ldc(u8),
    Nop,
    Return,
}

impl Instruction {
    pub fn from<F: ByteUtils>(file: &mut F) -> std::io::Result<Instruction> {
        let opcode = file.read_u1()?;
        Ok(match opcode {
            0x0 => Instruction::Nop,
            0xb1 => Instruction::Return,
            0x2a => Instruction::ALoad0,
            0x2b => Instruction::ALoad1,
            0x2c => Instruction::ALoad2,
            0x2d => Instruction::ALoad3,
            0x3b => Instruction::IStore0,
            0x3c => Instruction::IStore1,
            0x3d => Instruction::IStore2,
            0x3e => Instruction::IStore3,
            0x1a => Instruction::ILoad0,
            0x1b => Instruction::ILoad1,
            0x1c => Instruction::ILoad2,
            0x1d => Instruction::ILoad3,
            0x60 => Instruction::IAdd,
            0x10 => Instruction::BiPush(file.read_u1()?),
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
                panic!("Unrecognized opcode: {opcode:#x}");
            }
        })
    }
}
