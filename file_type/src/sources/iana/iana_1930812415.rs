use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1930812415: FileFormat = FileFormat {
    id: 1_930_812_415,
    source_type: SourceType::Iana,
    name: "simple-message-summary",
    extensions: &[],
    media_types: &["application/simple-message-summary"],
    signatures: &[],
    related_formats: &[],
};
