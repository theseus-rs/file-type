use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_29465275: FileFormat = FileFormat {
    id: 29_465_275,
    source_type: SourceType::Iana,
    name: "vnd.trueapp",
    extensions: &[],
    media_types: &["application/vnd.trueapp"],
    internal_signatures: &[],
    related_formats: &[],
};
