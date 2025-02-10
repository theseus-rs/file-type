use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2769996487: FileType = FileType {
    file_format: &FileFormat {
        id: 2_769_996_487,
        source_type: SourceType::Httpd,
        name: "apple mpegurl",
        extensions: &["m3u8"],
        media_types: &["application/vnd.apple.mpegurl"],
        signatures: &[],
        related_formats: &[],
    },
};
