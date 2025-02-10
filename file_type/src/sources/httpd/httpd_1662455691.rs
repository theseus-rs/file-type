use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1662455691: FileType = FileType {
    file_format: &FileFormat {
        id: 1_662_455_691,
        source_type: SourceType::Httpd,
        name: "gpx xml",
        extensions: &["gpx"],
        media_types: &["application/gpx+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
