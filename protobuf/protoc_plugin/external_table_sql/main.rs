use std::collections::HashMap;

use prost_types::{
    compiler::{code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse},
    field_descriptor_proto::{Label, Type},
    FileDescriptorProto,
};

fn main() -> anyhow::Result<()> {
    protoc_plugin::run(ExternalTableSqlGenerator::default())
}

#[derive(Debug, Default, Clone)]
struct ExternalTableSqlGenerator {}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Table {
    name: String,
    columns: Vec<Column>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Column {
    name: String,
    label: Label,
    r#type: Type,
    type_name: Option<String>,
}

impl protoc_plugin::CodeGenerator for ExternalTableSqlGenerator {
    fn gen_code(&self, req: CodeGeneratorRequest) -> CodeGeneratorResponse {
        eprintln!("{:?}", req.parameter());

        let mut resp = CodeGeneratorResponse::default();

        let file_desc_protos =
            req.proto_file.iter().map(|fd| (fd.name(), fd)).collect::<HashMap<_, _>>();

        // let type_desc_protos: HashMap<&str, &DescriptorProto> = unimplemented!();
        // let enum_desc_protos: HashMap<&str, &DescriptorProto> = unimplemented!();

        for f in &req.file_to_generate {
            let file_desc = file_desc_protos[f.as_str()];
            for type_desc in &file_desc.message_type {
                let name = type_desc.name().to_owned();
                let columns = type_desc
                    .field
                    .iter()
                    .map(|field| Column {
                        name: field.name().to_owned(),
                        label: field.label(),
                        r#type: field.r#type(),
                        type_name: field.type_name.clone(),
                    })
                    .collect();

                let table = Table { name, columns };
                eprintln!("{:#?}", table);
            }
        }

        resp.file.extend(req.file_to_generate.iter().map(|f| {
            let stem = f.strip_suffix(".proto").expect("no .proto suffix");
            code_generator_response::File {
                name: Some(format!("{}.external_table.sql", stem)),
                content: None,
                ..Default::default()
            }
        }));

        resp
    }
}
