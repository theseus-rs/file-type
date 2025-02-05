use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_939688699: FileFormat = FileFormat {
    id: 939_688_699,
    source_type: SourceType::Httpd,
    name: "ogg",
    extensions: &["ogv"],
    media_types: &["video/ogg"],
    signatures: &[],
    related_formats: &[],
};
