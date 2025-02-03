use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10466185904492655952: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "vrml",
    extensions: &["wrl", "vrml"],
    media_types: &["model/vrml"],
    internal_signatures: &[],
    related_formats: &[],
};
