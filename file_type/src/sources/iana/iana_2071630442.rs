use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2071630442: FileFormat = FileFormat {
    id: 2_071_630_442,
    source_type: SourceType::Iana,
    name: "vnd.hsl",
    extensions: &[],
    media_types: &["application/vnd.hsl"],
    internal_signatures: &[],
    related_formats: &[],
};
