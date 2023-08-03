use std::collections::HashMap;

use prost_reflect::DescriptorPool;
use prost_types::{
    compiler::{code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse},
    field_descriptor_proto::{Label, Type},
    FileDescriptorProto, FileDescriptorSet,
};

fn main() -> anyhow::Result<()> {
    protoc_plugin::run(ExternalTableSqlGenerator::default())
}

#[derive(Debug, Default, Clone)]
struct ExternalTableSqlGenerator {}

impl protoc_plugin::CodeGenerator for ExternalTableSqlGenerator {
    fn gen_code(&self, req: CodeGeneratorRequest) -> CodeGeneratorResponse {
        let pool = {
            let fd_set = FileDescriptorSet { file: req.proto_file.clone() };
            DescriptorPool::from_file_descriptor_set(fd_set)
                .expect("failed to create descriptor pool")
        };

        let file = req
            .file_to_generate
            .iter()
            .map(|file| {
                let file_desc = pool
                    .get_file_by_name(file.as_str())
                    .expect("failed to fetch a file descriptor from a pool");

                let mut buf = String::with_capacity(1 << 10);
                for msg_desc in file_desc.messages() {
                    // buf.push_str(&format!("{:#?}\n", msg_desc));

                    for field_desc in msg_desc.fields() {
                        buf.push_str(&format!(
                            "{name}: {kind:?}\n",
                            name = field_desc.name(),
                            kind = field_desc.kind()
                        ));
                    }
                }

                let stem = file.strip_suffix(".proto").expect("no .proto suffix");
                code_generator_response::File {
                    name: Some(format!("{}.external_table.sql", stem)),
                    content: Some(buf),
                    ..Default::default()
                }
            })
            .collect();

        CodeGeneratorResponse { file, ..Default::default() }
    }
}
