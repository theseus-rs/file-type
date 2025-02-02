use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13507959258524992275: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "x3d vrml",
    extensions: &["x3dv", "x3dvz"],
    media_types: &["model/x3d+vrml"],
    internal_signatures: &[],
    related_formats: &[],
};
