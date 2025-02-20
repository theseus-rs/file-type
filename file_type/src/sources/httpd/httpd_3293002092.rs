use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3293002092: FileType = FileType {
    file_format: &FileFormat {
        id: 3_293_002_092,
        source_type: SourceType::Httpd,
        name: "portable graymap",
        extensions: &["pgm"],
        media_types: &["image/x-portable-graymap"],
        signatures: &[],
        related_formats: &[],
    },
};
