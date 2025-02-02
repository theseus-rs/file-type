use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2323420538982214786: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "x3d xml",
    extensions: &["x3d", "x3dz"],
    media_types: &["model/x3d+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
