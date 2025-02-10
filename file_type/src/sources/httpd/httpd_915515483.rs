use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_915515483: FileType = FileType {
    file_format: &FileFormat {
        id: 915_515_483,
        source_type: SourceType::Httpd,
        name: "html",
        extensions: &["html", "htm"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
