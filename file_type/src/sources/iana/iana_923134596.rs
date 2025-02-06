use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_923134596: FileFormat = FileFormat {
    id: 923_134_596,
    source_type: SourceType::Iana,
    name: "vnd.apple.numbers",
    extensions: &[],
    media_types: &["application/vnd.apple.numbers"],
    signatures: &[],
    related_formats: &[],
};
