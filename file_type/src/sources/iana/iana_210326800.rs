use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_210326800: FileFormat = FileFormat {
    id: 210_326_800,
    source_type: SourceType::Iana,
    name: "VP8",
    extensions: &[],
    media_types: &["video/VP8"],
    internal_signatures: &[],
    related_formats: &[],
};
