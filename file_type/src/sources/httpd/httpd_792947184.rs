use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_792947184: FileFormat = FileFormat {
    id: 792_947_184,
    source_type: SourceType::Httpd,
    name: "vrml",
    extensions: &["wrl", "vrml"],
    media_types: &["model/vrml"],
    internal_signatures: &[],
    related_formats: &[],
};
