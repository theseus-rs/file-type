use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3782623085: FileType = FileType {
    file_format: &FileFormat {
        id: 3_782_623_085,
        source_type: SourceType::Httpd,
        name: "atom xml",
        extensions: &["atom"],
        media_types: &["application/atom+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
