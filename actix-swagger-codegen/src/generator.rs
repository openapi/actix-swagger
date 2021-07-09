use openapiv3::OpenAPI;

pub struct CodeGenerator<'a> {
    package: String,
    schema: OpenAPI,
    buf: &'a mut str,
    depth: u8,
}

impl<'a> CodeGenerator<'a> {
    pub fn path_responses(&mut self) -> String {

        for operation in response.iter() {
            self.buf.push_str("pub enum ")
        }

        self.buf.to_string()
    }

    fn push_indent(&mut self) {
        for _ in 0..self.depth {
            self.buf.push_str("    ");
        }
    }
}