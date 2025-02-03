use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1598516405: FileFormat = FileFormat {
    id: 1_598_516_405,
    source_type: SourceType::Iana,
    name: "xop+xml",
    extensions: &[],
    media_types: &["application/xop+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
