use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2097038630: FileFormat = FileFormat {
    id: 2_097_038_630,
    source_type: SourceType::Iana,
    name: "markdown",
    extensions: &[],
    media_types: &["text/markdown"],
    signatures: &[],
    related_formats: &[],
};
