use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_426516203: FileFormat = FileFormat {
    id: 426_516_203,
    source_type: SourceType::Iana,
    name: "vnd.semd",
    extensions: &[],
    media_types: &["application/vnd.semd"],
    signatures: &[],
    related_formats: &[],
};
