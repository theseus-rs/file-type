use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3962761714: FileType = FileType {
    file_format: &FileFormat {
        id: 3_962_761_714,
        source_type: SourceType::Httpd,
        name: "osgeo mapguide package",
        extensions: &["mgp"],
        media_types: &["application/vnd.osgeo.mapguide.package"],
        signatures: &[],
        related_formats: &[],
    },
};
