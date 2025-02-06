use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_379983263: FileFormat = FileFormat {
    id: 379_983_263,
    source_type: SourceType::Iana,
    name: "aif+cbor",
    extensions: &[],
    media_types: &["application/aif+cbor"],
    signatures: &[],
    related_formats: &[],
};
