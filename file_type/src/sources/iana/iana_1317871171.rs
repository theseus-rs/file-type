use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1317871171: FileFormat = FileFormat {
    id: 1_317_871_171,
    source_type: SourceType::Iana,
    name: "vnd.fmi.flexstor",
    extensions: &[],
    media_types: &["text/vnd.fmi.flexstor"],
    signatures: &[],
    related_formats: &[],
};
