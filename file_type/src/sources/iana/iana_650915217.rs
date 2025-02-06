use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_650915217: FileFormat = FileFormat {
    id: 650_915_217,
    source_type: SourceType::Iana,
    name: "UEMCLIP",
    extensions: &[],
    media_types: &["audio/UEMCLIP"],
    signatures: &[],
    related_formats: &[],
};
