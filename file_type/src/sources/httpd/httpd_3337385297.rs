use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3337385297: FileType = FileType {
    file_format: &FileFormat {
        id: 3_337_385_297,
        source_type: SourceType::Httpd,
        name: "timestamped data",
        extensions: &["tsd"],
        media_types: &["application/timestamped-data"],
        signatures: &[],
        related_formats: &[],
    },
};
