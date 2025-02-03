use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6898497964742600926: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "x3d binary",
    extensions: &["x3db", "x3dbz"],
    media_types: &["model/x3d+binary"],
    internal_signatures: &[],
    related_formats: &[],
};
