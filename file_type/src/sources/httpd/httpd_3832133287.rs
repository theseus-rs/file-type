use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3832133287: FileType = FileType {
    file_format: &FileFormat {
        id: 3_832_133_287,
        source_type: SourceType::Httpd,
        name: "xv xml",
        extensions: &["mxml", "xhvml", "xvml", "xvm"],
        media_types: &["application/xv+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
