use crate::raw::*;
use std::fs::File;

#[derive(Debug, Default)]
pub struct MethodInfo {
    pub access_flags: AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

impl MethodInfo {
    pub fn from(file: &mut File) -> std::io::Result<MethodInfo> {
        let access_flags = AccessFlags::from(file.read_u2()?);
        let name_index = file.read_u2()?;
        let descriptor_index = file.read_u2()?;
        let attributes_count = file.read_u2()?;
        let attributes = file.read_array(attributes_count.into(), Attribute::from)?;

        Ok(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}
