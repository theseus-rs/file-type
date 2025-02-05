use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_191805810: FileFormat = FileFormat {
    id: 191_805_810,
    source_type: SourceType::Iana,
    name: "vnd.banana-accounting",
    extensions: &[],
    media_types: &["application/vnd.banana-accounting"],
    signatures: &[],
    related_formats: &[],
};
