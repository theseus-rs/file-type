use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1261369801: FileFormat = FileFormat {
    id: 1_261_369_801,
    source_type: SourceType::Iana,
    name: "ogg",
    extensions: &[],
    media_types: &["application/ogg"],
    signatures: &[],
    related_formats: &[],
};
