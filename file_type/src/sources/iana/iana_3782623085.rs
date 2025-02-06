use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3782623085: FileFormat = FileFormat {
    id: 3_782_623_085,
    source_type: SourceType::Iana,
    name: "atom+xml",
    extensions: &[],
    media_types: &["application/atom+xml"],
    signatures: &[],
    related_formats: &[],
};
