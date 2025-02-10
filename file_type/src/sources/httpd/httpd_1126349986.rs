use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1126349986: FileType = FileType {
    file_format: &FileFormat {
        id: 1_126_349_986,
        source_type: SourceType::Httpd,
        name: "pcx",
        extensions: &["pcx"],
        media_types: &["image/x-pcx"],
        signatures: &[],
        related_formats: &[],
    },
};
