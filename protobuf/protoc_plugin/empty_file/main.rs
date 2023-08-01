use std::io;
use std::io::{Read, Write};

use anyhow::Context;
use prost::Message;
use prost_types::compiler::code_generator_response;
use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};

fn main() -> anyhow::Result<()> {
    let mut buf = Vec::with_capacity(1 << 10);
    let n = io::stdin().read_to_end(&mut buf).context("failed to read proto message from stdin")?;
    let req = CodeGeneratorRequest::decode(&buf[..n])?;

    let empty_file_gen = EmptyFileGenerator::default();
    let resp = empty_file_gen.gen_code(req)?;

    buf.clear();
    resp.encode(&mut buf).context("failed to encode a response message")?;

    io::stdout().write_all(&buf)?;

    Ok(())
}

trait CodeGenerator {
    type Error: std::error::Error;
    fn gen_code(&self, req: CodeGeneratorRequest) -> Result<CodeGeneratorResponse, Self::Error>;
}

#[derive(Default, Clone)]
struct EmptyFileGenerator {}

impl CodeGenerator for EmptyFileGenerator {
    type Error = io::Error;

    fn gen_code(&self, req: CodeGeneratorRequest) -> Result<CodeGeneratorResponse, Self::Error> {
        let mut resp = CodeGeneratorResponse::default();

        resp.file.extend(req.file_to_generate.iter().map(|f| {
            let stem = f.strip_suffix(".proto").expect("no .proto suffix");
            let mut file = code_generator_response::File::default();
            file.name = Some(format!("{}.empty", stem));
            file.content = None;
            file
        }));

        Ok(resp)
    }
}
