use crate::raw::*;
use std::fs::File;
use std::io::Result;

#[derive(Debug, Default)]
pub struct Class {
    pub major: u16,
    pub minor: u16,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_info: AccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<Attribute>,
}

impl Class {
    pub fn from(mut file: File) -> Result<Class> {
        let magic = file.read_u4()?;
        assert!(magic == 0xcafebabe);

        let mut class = Class::default();
        class.minor = file.read_u2()?;
        class.major = file.read_u2()?;
        let constant_pool_count = file.read_u2()?;
        class.constant_pool =
            file.read_array((constant_pool_count - 1).into(), ConstantPoolInfo::from)?;

        class.access_info = AccessFlags::from(file.read_u2()?);
        class.this_class = file.read_u2()?;
        class.super_class = file.read_u2()?;

        let interfaces_count = file.read_u2()?;
        assert!(interfaces_count == 0); // Interfaces support is not implemented yet

        let fields_count = file.read_u2()?;
        assert!(fields_count == 0); // Fields support is not implemented yet

        let methods_count = file.read_u2()?;
        class.methods = file.read_array(methods_count.into(), MethodInfo::from)?;

        let attributes_count = file.read_u2()?;
        class.attributes = file.read_array(attributes_count.into(), Attribute::from)?;

        return Ok(class);
    }

    pub fn resolve_attributes(&mut self) -> std::io::Result<()> {
        let attributes = self.attributes.iter_mut().chain(
            self.methods
                .iter_mut()
                .flat_map(|method| method.attributes.iter_mut()),
        );

        resolve_attributes(&self.constant_pool, attributes)
    }

    pub fn disassemble(&self) {
        let class_name = self.constant_pool[self.this_class as usize - 1]
            .name(&self.constant_pool)
            .expect("This class should be pointing to valid class name");
        println!("class {class_name}");
        println!("{{");

        for method in &self.methods {
            let mut name = self.constant_pool[method.name_index as usize - 1].utf8().expect("Method should be pointing to valid method name");
            if name == "<init>" {
                name = class_name.clone();
            }
            println!("  {name}()");
        }

        println!("}}");
    }
}
