use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4088769950: FileFormat = FileFormat {
    id: 4_088_769_950,
    source_type: SourceType::Iana,
    name: "sql",
    extensions: &[],
    media_types: &["application/sql"],
    signatures: &[],
    related_formats: &[],
};
