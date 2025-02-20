use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_354700864: FileType = FileType {
    file_format: &FileFormat {
        id: 354_700_864,
        source_type: SourceType::Httpd,
        name: "mpegurl",
        extensions: &["m3u"],
        media_types: &["audio/x-mpegurl"],
        signatures: &[],
        related_formats: &[],
    },
};
