use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_237827459: FileFormat = FileFormat {
    id: 237_827_459,
    source_type: SourceType::Iana,
    name: "imdn+xml",
    extensions: &[],
    media_types: &["message/imdn+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
