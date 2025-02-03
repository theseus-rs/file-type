use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2984238633: FileFormat = FileFormat {
    id: 2_984_238_633,
    source_type: SourceType::Httpd,
    name: "x3d xml",
    extensions: &["x3d", "x3dz"],
    media_types: &["model/x3d+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
