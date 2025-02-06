use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3248699242: FileFormat = FileFormat {
    id: 3_248_699_242,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"],
    signatures: &[],
    related_formats: &[],
};
