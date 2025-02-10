use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2518261554: FileType = FileType {
    file_format: &FileFormat {
        id: 2_518_261_554,
        source_type: SourceType::Httpd,
        name: "f4v",
        extensions: &["f4v"],
        media_types: &["video/x-f4v"],
        signatures: &[],
        related_formats: &[],
    },
};
