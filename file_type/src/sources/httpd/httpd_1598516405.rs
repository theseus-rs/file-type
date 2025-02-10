use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1598516405: FileType = FileType {
    file_format: &FileFormat {
        id: 1_598_516_405,
        source_type: SourceType::Httpd,
        name: "xop xml",
        extensions: &["xop"],
        media_types: &["application/xop+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
