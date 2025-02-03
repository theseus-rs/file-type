use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3962761714: FileFormat = FileFormat {
    id: 3_962_761_714,
    source_type: SourceType::Httpd,
    name: "osgeo mapguide package",
    extensions: &["mgp"],
    media_types: &["application/vnd.osgeo.mapguide.package"],
    internal_signatures: &[],
    related_formats: &[],
};
