use crate::raw::*;
use num_enum::TryFromPrimitive;
use std::fs::File;
use std::io::Read;

#[derive(TryFromPrimitive)]
#[repr(u8)]
enum ConstantPoolInfoTag {
    Class = 7,
    FieldRef = 9,
    MethodRef = 10,
    NameAndType = 12,
    String = 8,
    Utf8 = 1,
}

// aka cp_info
#[derive(Debug)]
pub enum ConstantPoolInfo {
    Class {
        name_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    String {
        string_index: u16,
    },
    Utf8(String),
}

impl ConstantPoolInfo {
    pub fn from(file: &mut File) -> std::io::Result<ConstantPoolInfo> {
        let tag = file.read_u1()?;
        if let Ok(tag) = ConstantPoolInfoTag::try_from(tag) {
            Ok(match tag {
                ConstantPoolInfoTag::Class => ConstantPoolInfo::Class {
                    name_index: file.read_u2()?,
                },
                ConstantPoolInfoTag::MethodRef => ConstantPoolInfo::MethodRef {
                    class_index: file.read_u2()?,
                    name_and_type_index: file.read_u2()?,
                },
                ConstantPoolInfoTag::FieldRef => ConstantPoolInfo::FieldRef {
                    class_index: file.read_u2()?,
                    name_and_type_index: file.read_u2()?,
                },
                ConstantPoolInfoTag::NameAndType => ConstantPoolInfo::NameAndType {
                    name_index: file.read_u2()?,
                    descriptor_index: file.read_u2()?,
                },
                ConstantPoolInfoTag::String => ConstantPoolInfo::String {
                    string_index: file.read_u2()?,
                },
                ConstantPoolInfoTag::Utf8 => {
                    let length = file.read_u2()?;
                    let mut buf = vec![0u8; length.into()];
                    file.read_exact(buf.as_mut_slice())?;
                    // SAFETY: We belive that Java compilers know how to generate valid utf8
                    // strings
                    ConstantPoolInfo::Utf8(unsafe { String::from_utf8_unchecked(buf) })
                }
            })
        } else {
            println!("Skipping tag {tag} due to not beeing implemented yet");
            panic!();
        }
    }

    pub fn name(&self, constant_pool: &[ConstantPoolInfo]) -> Option<String> {
        match self {
            Self::Class { name_index } => {
                if let ConstantPoolInfo::Utf8(name) = &constant_pool[*name_index as usize - 1] {
                    Some(name.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn utf8(&self) -> Option<String> {
        if let ConstantPoolInfo::Utf8(utf8) = &self {
            Some(utf8.to_string())
        } else {
            None
        }
    }
}
