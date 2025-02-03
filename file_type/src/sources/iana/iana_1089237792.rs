use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1089237792: FileFormat = FileFormat {
    id: 1_089_237_792,
    source_type: SourceType::Iana,
    name: "postscript",
    extensions: &[],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
