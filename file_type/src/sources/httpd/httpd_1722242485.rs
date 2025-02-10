use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1722242485: FileType = FileType {
    file_format: &FileFormat {
        id: 1_722_242_485,
        source_type: SourceType::Httpd,
        name: "google earth kmz",
        extensions: &["kmz"],
        media_types: &["application/vnd.google-earth.kmz"],
        signatures: &[],
        related_formats: &[],
    },
};
