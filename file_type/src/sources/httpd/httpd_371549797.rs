use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_371549797: FileType = FileType {
    file_format: &FileFormat {
        id: 371_549_797,
        source_type: SourceType::Httpd,
        name: "dece video",
        extensions: &["uvv", "uvvv"],
        media_types: &["video/vnd.dece.video"],
        signatures: &[],
        related_formats: &[],
    },
};
