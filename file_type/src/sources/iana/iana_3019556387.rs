use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3019556387: FileFormat = FileFormat {
    id: 3_019_556_387,
    source_type: SourceType::Iana,
    name: "thraud+xml",
    extensions: &[],
    media_types: &["application/thraud+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
