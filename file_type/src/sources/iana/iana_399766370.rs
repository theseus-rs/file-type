use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_399766370: FileFormat = FileFormat {
    id: 399_766_370,
    source_type: SourceType::Iana,
    name: "vnd.ms-powerpoint.slide.macroEnabled.12",
    extensions: &[],
    media_types: &["application/vnd.ms-powerpoint.slide.macroEnabled.12"],
    signatures: &[],
    related_formats: &[],
};
