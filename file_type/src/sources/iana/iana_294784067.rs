use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_294784067: FileFormat = FileFormat {
    id: 294_784_067,
    source_type: SourceType::Iana,
    name: "vnd.mermaid",
    extensions: &[],
    media_types: &["application/vnd.mermaid"],
    signatures: &[],
    related_formats: &[],
};
