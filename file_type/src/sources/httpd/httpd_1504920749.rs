use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1504920749: FileType = FileType {
    file_format: &FileFormat {
        id: 1_504_920_749,
        source_type: SourceType::Httpd,
        name: "google earth kml xml",
        extensions: &["kml"],
        media_types: &["application/vnd.google-earth.kml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
