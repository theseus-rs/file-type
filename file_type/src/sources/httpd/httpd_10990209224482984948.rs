use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10990209224482984948: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ogg",
    extensions: &["ogv"],
    media_types: &["video/ogg"],
    internal_signatures: &[],
    related_formats: &[],
};
