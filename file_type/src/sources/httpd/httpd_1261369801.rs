use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1261369801: FileFormat = FileFormat {
    id: 1_261_369_801,
    source_type: SourceType::Httpd,
    name: "ogg",
    extensions: &["ogx"],
    media_types: &["application/ogg"],
    internal_signatures: &[],
    related_formats: &[],
};
