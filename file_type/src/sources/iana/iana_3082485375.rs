use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3082485375: FileFormat = FileFormat {
    id: 3_082_485_375,
    source_type: SourceType::Iana,
    name: "vnd.datalog",
    extensions: &[],
    media_types: &["application/vnd.datalog"],
    signatures: &[],
    related_formats: &[],
};
