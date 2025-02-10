use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_483127336: FileType = FileType {
    file_format: &FileFormat {
        id: 483_127_336,
        source_type: SourceType::Httpd,
        name: "mpeg",
        extensions: &["mpga", "mp2", "mp2a", "mp3", "m2a", "m3a"],
        media_types: &["audio/mpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
