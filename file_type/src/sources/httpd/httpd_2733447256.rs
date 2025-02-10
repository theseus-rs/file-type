use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2733447256: FileType = FileType {
    file_format: &FileFormat {
        id: 2_733_447_256,
        source_type: SourceType::Httpd,
        name: "crick clicker wordbank",
        extensions: &["clkw"],
        media_types: &["application/vnd.crick.clicker.wordbank"],
        signatures: &[],
        related_formats: &[],
    },
};
