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

        println!("this_class {class_name}");

        for (i, constant) in self.constant_pool.iter().enumerate() {
            print!("const {i:3} ");
            dump_constant(&self.constant_pool, constant);
        }

        for method in &self.methods {
            let method_name = self.constant_pool[method.name_index as usize - 1]
                .utf8()
                .expect("Method should be pointing to valid method name");
            println!("method {method_name}");
            for attr in &method.attributes {
                match &attr.info {
                    AttributeInfo::Code {
                        max_stack,
                        max_locals,
                        code,
                        exception_table,
                        attributes,
                    } => {
                        println!(
                            "  attribute Code max_stack={max_stack} max_locals={max_locals}"
                        );

                        for attr in attributes {
                            match attr.info {
                                // TODO: We could use it
                                AttributeInfo::LineNumberTable( .. ) => continue,

                                _ => todo!(),
                            }
                        }

                        for instruction in code {
                            print!("    ");
                            match instruction {
                                Instruction::Nop => println!("nop"),
                                Instruction::Return => println!("return"),
                                Instruction::IStore0 => println!("istore_0"),
                                Instruction::IStore1 => println!("istore_1"),
                                Instruction::IStore2 => println!("istore_2"),
                                Instruction::IStore3 => println!("istore_3"),
                                Instruction::ALoad0 => println!("aload_0"),
                                Instruction::ALoad1 => println!("aload_1"),
                                Instruction::ALoad2 => println!("aload_2"),
                                Instruction::ALoad3 => println!("aload_3"),
                                Instruction::ILoad0 => println!("iload_0"),
                                Instruction::ILoad1 => println!("iload_1"),
                                Instruction::ILoad2 => println!("iload_2"),
                                Instruction::ILoad3 => println!("iload_3"),
                                Instruction::IAdd => println!("iadd"),
                                Instruction::BiPush(b) => println!("bipush {b} // {b:#04x}"),
                                Instruction::Ldc(constant) => {
                                    print!("ldc {constant}");
                                    if (*constant as usize) < self.constant_pool.len() {
                                        print!(" // ");
                                        dump_constant(&self.constant_pool, &self.constant_pool[*constant as usize - 1]);
                                    } else {
                                        println!(" // couldn't resolve constant");
                                    }
                                },
                                Instruction::GetStatic(index) => {
                                    print!("getstatic {index}");
                                    if (*index as usize) < self.constant_pool.len() {
                                        print!(" // ");
                                        dump_constant(&self.constant_pool, &self.constant_pool[*index as usize - 1]);
                                    } else {
                                        println!(" // couldn't resolve constant");
                                    }
                                }
                                Instruction::InvokeSpecial(method) => {
                                    print!("invokespecial {method}");
                                    if (*method as usize) < self.constant_pool.len() {
                                        print!(" // ");
                                        dump_constant(&self.constant_pool, &self.constant_pool[*method as usize - 1]);
                                    } else {
                                        println!(" // couldn't resolve constant");
                                    }
                                }
                                Instruction::InvokeVirtual(method) => {
                                    print!("invokevirtual {method}");
                                    if (*method as usize) < self.constant_pool.len() {
                                        print!(" // ");
                                        dump_constant(&self.constant_pool, &self.constant_pool[*method as usize - 1]);
                                    } else {
                                        println!(" // couldn't resolve constant");
                                    }
                                }
                            }
                        }

                        if exception_table.len() > 0 {
                            todo!();
                        }
                    }
                    _ => todo!(),
                }
            }
        }
    }
}

pub fn dump_constant(constant_pool: &Vec<ConstantPoolInfo>,  constant: &ConstantPoolInfo) {
            match constant {
                ConstantPoolInfo::FieldRef {
                    class_index,
                    name_and_type_index,
                } => {
                    let ConstantPoolInfo::Class { name_index } =
                        constant_pool[*class_index as usize - 1]
                    else {
                        unreachable!("Field.class_index must point to class")
                    };
                    let class_name = constant_pool[name_index as usize - 1].utf8().unwrap();

                    let ConstantPoolInfo::NameAndType {
                        name_index,
                        descriptor_index,
                    } = constant_pool[*name_and_type_index as usize - 1]
                    else {
                        unreachable!("Field.name_and_type_index must point to NameAndType");
                    };
                    let field_name = constant_pool[name_index as usize - 1].utf8().unwrap();

                    let ConstantPoolInfo::Utf8(descriptor) =
                        &constant_pool[descriptor_index as usize - 1]
                    else {
                        unreachable!("NameAndType.descriptor_index must point to the Utf8");
                    };

                    println!(
                        "fieldref class={class_name} field={field_name} descriptor={descriptor:?}"
                    );
                }
                ConstantPoolInfo::NameAndType {
                    name_index,
                    descriptor_index,
                } => {
                    let name = constant_pool[*name_index as usize - 1].utf8().unwrap();

                    let ConstantPoolInfo::Utf8(descriptor) =
                        &constant_pool[*descriptor_index as usize - 1]
                    else {
                        unreachable!("NameAndType.descriptor_index must point to the Utf8");
                    };

                    println!("name={name} type={descriptor:?}");
                }
                ConstantPoolInfo::MethodRef {
                    class_index,
                    name_and_type_index,
                } => {
                    let ConstantPoolInfo::Class { name_index } =
                        constant_pool[*class_index as usize - 1]
                    else {
                        unreachable!("MethodRef.class_index must point to class")
                    };
                    let class_name = constant_pool[name_index as usize - 1].utf8().unwrap();

                    let ConstantPoolInfo::NameAndType {
                        name_index,
                        descriptor_index,
                    } = constant_pool[*name_and_type_index as usize - 1]
                    else {
                        unreachable!("Method.name_and_type_index must point to NameAndType");
                    };
                    let method_name = constant_pool[name_index as usize - 1].utf8().unwrap();

                    let ConstantPoolInfo::Utf8(descriptor) =
                        &constant_pool[descriptor_index as usize - 1]
                    else {
                        unreachable!("NameAndType.descriptor_index must point to the Utf8");
                    };

                    println!("methodref class={class_name} method={method_name} descriptor={descriptor:?}");
                }
                ConstantPoolInfo::Class { name_index } => {
                    let name = constant_pool[*name_index as usize - 1].utf8().unwrap();
                    println!("class {name}");
                }
                ConstantPoolInfo::String { string_index } => {
                    let str = constant_pool[*string_index as usize - 1]
                        .utf8()
                        .unwrap();
                    println!("string {str:?}");
                }
                ConstantPoolInfo::Utf8(str) => println!("{str:?}"),
            }
}
