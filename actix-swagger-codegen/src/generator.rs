use openapiv3::{OpenAPI, ReferenceOr, PathItem, Operation, Paths};
use crate::utils::parse_schema;
use std::path::PathBuf;
use convert_case::{Casing, Case};

#[derive(Debug)]
pub struct CodeGenerator<'a> {
    schema: OpenAPI,
    buf: &'a mut String,
    depth: u8,
    path: Vec<i32>,
    //target: PathBuf,
}

impl<'a> CodeGenerator<'a> {
    pub(crate) fn new(buf: &'a mut String, schema: OpenAPI) -> Self {
        Self {
            schema,
            depth: 0,
            path: Vec::new(),
            buf,
            //target: std::env::var_os("OUT_DIR").unwrap().into()
        }
    }

    // fn write(&self) {
    //     std::fs::write(&self.target, &self.buf);
    // }

    pub(crate) fn generate(&mut self) {
        self.push_routes(self.schema.paths.clone());
    }

    fn push_indent(&mut self) {
        for _ in 0..self.depth {
            self.buf.push_str("    ");
        }
    }

    fn push_routes(&mut self, paths: Paths) {
        for path in paths.values() {
            let path: &ReferenceOr<PathItem> = path;

            match path {
                ReferenceOr::Item(item) => {
                    for operation in item.iter() {
                        self.push_operation(operation);
                    }
                },
                ReferenceOr::Reference {
                    reference
                } => {
                    // TODO: reference finding
                }
            }
        }
    }

    fn push_operation(&mut self, operation: &Operation) {
        self.push_mod(&operation.operation_id.as_ref().unwrap().to_case(Case::Snake));
        self.pop_mod();
        self.buf.push_str("\n");
    }

    fn push_mod(&mut self, module: &str) {
        self.push_indent();

        self.buf.push_str("pub mod ");
        self.buf.push_str(&module.to_case(Case::Snake));
        self.buf.push_str(" {\n");

        self.depth +=1;
    }

    fn pop_mod(&mut self) {
        self.depth -= 1;

        self.push_indent();
        self.buf.push_str("}\n");
    }
}

#[cfg(test)]
mod tests {
    use crate::{generate, parse_schema};
    use crate::generator::CodeGenerator;

    #[test]
    fn generates() {
        let schema = parse_schema().unwrap();
        let mut buf = String::new();
        let mut generator = CodeGenerator::new(&mut buf, schema);
        generator.generate();

        println!("{}", buf);
    }
}