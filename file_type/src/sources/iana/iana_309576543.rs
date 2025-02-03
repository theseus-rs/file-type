use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_309576543: FileFormat = FileFormat {
    id: 309_576_543,
    source_type: SourceType::Iana,
    name: "jxsi",
    extensions: &[],
    media_types: &["image/jxsi"],
    internal_signatures: &[],
    related_formats: &[],
};
