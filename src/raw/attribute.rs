use crate::raw::*;

#[derive(Debug)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug)]
pub enum AttributeInfo {
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<Instruction>,
        exception_table: Vec<Exception>,
        attributes: Vec<Attribute>,
    },
    LineNumberTable(Vec<LineNumber>),
    SourceFile(String),
    Uncrecognized(Vec<u8>),
}

#[derive(Debug)]
pub struct Attribute {
    pub name_index: u16,
    pub info: AttributeInfo,
}

impl Attribute {
    pub fn from<F: ByteUtils>(file: &mut F) -> std::io::Result<Attribute> {
        let name_index = file.read_u2()?;
        let attribute_length = file.read_u4()?;
        let mut info = vec![0u8; attribute_length as usize];
        file.read_exact(info.as_mut_slice())?;
        Ok(Attribute {
            name_index,
            info: AttributeInfo::Uncrecognized(info),
        })
    }
}

impl LineNumber {
    pub fn from<F: ByteUtils>(file: &mut F) -> std::io::Result<LineNumber> {
        Ok(LineNumber {
            start_pc: file.read_u2()?,
            line_number: file.read_u2()?,
        })
    }
}

impl AttributeInfo {
    pub fn from(
        constant_pool: &[ConstantPoolInfo],
        name: &str,
        mut bytes: &[u8],
    ) -> std::io::Result<Option<AttributeInfo>> {
        Ok(match name {
            "Code" => {
                let max_stack = bytes.read_u2()?;
                let max_locals = bytes.read_u2()?;

                let code_length = bytes.read_u4()? as usize;
                let mut code: Vec<Instruction> = vec![];
                let mut code_bytes = &bytes[..code_length];
                while !code_bytes.is_empty() {
                    code.push(Instruction::from(&mut code_bytes)?);
                }
                bytes = &bytes[code_length..];

                let exception_table_length = bytes.read_u2()?;
                assert!(exception_table_length == 0);

                let attributes_count = bytes.read_u2()?;
                let mut attributes = bytes.read_array(attributes_count.into(), Attribute::from)?;
                resolve_attributes(constant_pool, attributes.iter_mut())?;

                Some(AttributeInfo::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table: vec![],
                    attributes,
                })
            }
            "SourceFile" => {
                if let ConstantPoolInfo::Utf8(source) =
                    &constant_pool[bytes.read_u2()? as usize - 1]
                {
                    Some(AttributeInfo::SourceFile(source.to_string()))
                } else {
                    panic!();
                }
            }
            "LineNumberTable" => {
                let line_number_table_length = bytes.read_u2()?;
                Some(AttributeInfo::LineNumberTable(bytes.read_array(
                    line_number_table_length.into(),
                    LineNumber::from,
                )?))
            }
            _ => {
                println!("Unrecognized attribute name: {name}");
                None
            }
        })
    }
}

pub fn resolve_attributes<'a, It>(
    constant_pool: &[ConstantPoolInfo],
    attributes: It,
) -> std::io::Result<()>
where
    It: Iterator<Item = &'a mut Attribute>,
{
    for attribute in attributes {
        let name_index = attribute.name_index;

        let name =
            if let ConstantPoolInfo::Utf8(actual_name) = &constant_pool[name_index as usize - 1] {
                actual_name
            } else {
                panic!()
            };

        if let AttributeInfo::Uncrecognized(data) = &attribute.info {
            if let Some(info) = AttributeInfo::from(&constant_pool, &name, data.as_slice())? {
                attribute.info = info;
            }
        }
    }

    Ok(())
}
