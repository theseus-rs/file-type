use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_701141407755406764: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ogg",
    extensions: &["ogx"],
    media_types: &["application/ogg"],
    internal_signatures: &[],
    related_formats: &[],
};
