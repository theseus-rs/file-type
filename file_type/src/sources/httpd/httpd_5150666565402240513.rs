use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5150666565402240513: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "osgeo mapguide package",
    extensions: &["mgp"],
    media_types: &["application/vnd.osgeo.mapguide.package"],
    internal_signatures: &[],
    related_formats: &[],
};
