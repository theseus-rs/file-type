use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3930646908: FileFormat = FileFormat {
    id: 3_930_646_908,
    source_type: SourceType::Iana,
    name: "example",
    extensions: &[],
    media_types: &["application/example"],
    internal_signatures: &[],
    related_formats: &[],
};
