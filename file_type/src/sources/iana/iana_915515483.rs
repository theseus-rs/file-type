use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_915515483: FileFormat = FileFormat {
    id: 915_515_483,
    source_type: SourceType::Iana,
    name: "html",
    extensions: &[],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
