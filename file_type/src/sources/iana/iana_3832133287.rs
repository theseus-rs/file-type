use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3832133287: FileFormat = FileFormat {
    id: 3_832_133_287,
    source_type: SourceType::Iana,
    name: "xv+xml",
    extensions: &[],
    media_types: &["application/xv+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
