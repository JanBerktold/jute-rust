extern crate codegen;
extern crate jute_rust_parser;

use std::fs;
use std::io;

use codegen::Scope;
use jute_rust_parser::{FieldType, Module, Parser, PrimitiveFieldType};

pub struct Runner {
    files: Vec<String>,
    output_file: String,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            files: vec![],
            output_file: String::new(),
        }
    }

    pub fn add_file(&mut self, file: String) -> &mut Runner {
        self.files.push(file);
        self
    }

    pub fn set_output(&mut self, output: String) -> &mut Runner {
        self.output_file = output;
        self
    }

    pub fn run(&self) -> io::Result<()> {
        let mut generator = Generator {
            scope: Scope::new(),
        };

        for file in &self.files {
            let contents = fs::read_to_string(file)?;
            let mut parser = Parser::from_string(&contents);

            loop {
                match parser.next() {
                    Ok(module) => {
                        let code = generator.generate(&module);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        break;
                    }
                }
            }
        }

        let code = generator.to_string();
        let result = fs::write(&self.output_file, code);
        if result.is_err() {
            eprintln!("return");
            return result;
        }

        Ok(())
    }
}

pub struct Generator {
    scope: Scope,
}

impl Generator {
    fn generate(&mut self, module: &Module) {
        let rust_module = self
            .scope
            .get_or_new_module(&jute_module_to_rust(module.name.clone()));

        for class in &module.classes {
            let created_struct = rust_module.new_struct(&class.name).derive("Debug");

            for field in &class.fields {
                created_struct.field(
                    &format!("r#{}", field.name),
                    &jute_to_rust_type(field.field_type.clone()),
                );
            }
        }
    }

    fn to_string(&self) -> String {
        self.scope.to_string()
    }
}

fn jute_module_to_rust(name: String) -> String {
    str::replace(&name, ".", "_")
}

fn jute_to_rust_type(t: FieldType) -> String {
    match t {
        FieldType::Primitive(PrimitiveFieldType::Boolean) => String::from("bool"),
        FieldType::Primitive(PrimitiveFieldType::Buffer) => String::from("Vec<u8>"),
        FieldType::Primitive(PrimitiveFieldType::Byte) => String::from("u8"),
        FieldType::Primitive(PrimitiveFieldType::Double) => String::from("f64"),
        FieldType::Primitive(PrimitiveFieldType::Float) => String::from("f32"),
        FieldType::Primitive(PrimitiveFieldType::Int) => String::from("i32"),
        FieldType::Primitive(PrimitiveFieldType::Long) => String::from("i64"),
        FieldType::Primitive(PrimitiveFieldType::String) => String::from("String"),
        FieldType::Vector(t) => format!("Vec<{}>", jute_to_rust_type(FieldType::Primitive(t))),
        FieldType::Primitive(PrimitiveFieldType::Custom(name)) => jute_type_reference_to_rust(name),
        other => String::from("unknown"),
    }
}

fn jute_type_reference_to_rust(name: String) -> String {
    if let Some(pos) = name.rfind('.') {
        let (module_name, type_name) = name.split_at(pos);
        let type_name_without_dot = &type_name[1..type_name.len()];
        return format!(
            "{}::{}",
            jute_module_to_rust(String::from(module_name)),
            type_name_without_dot
        );
    }

    name
}
