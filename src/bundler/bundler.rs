use std::fs::File;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{Parser, StringInput, Syntax};

pub fn bundle_code(file_path: &str) {
    let code = std::fs::read_to_string(file_path).unwrap();
    let mut parser = Parser::new(Syntax::default(), StringInput::new(&code, 0, 0), None);
    let module = parser.parse_module().expect("Failed to parse");

    let mut buf = Vec::new();
    let mut emitter = Emitter {
        cfg: Default::default(),
        wr: JsWriter::new(Default::default(), "\n", &mut buf, None),
        comments: None,
    };

    emitter.emit_module(&module).expect("Failed to emit");
    std::fs::write("output.js", buf).unwrap();
}
