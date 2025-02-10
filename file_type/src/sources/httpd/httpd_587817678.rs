use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_587817678: FileType = FileType {
    file_format: &FileFormat {
        id: 587_817_678,
        source_type: SourceType::Httpd,
        name: "nfo",
        extensions: &["nfo"],
        media_types: &["text/x-nfo"],
        signatures: &[],
        related_formats: &[],
    },
};
