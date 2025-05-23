use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3002875504: FileType = FileType {
    file_format: &FileFormat {
        id: 3_002_875_504,
        source_type: SourceType::Httpd,
        name: "uuencode",
        extensions: &["uu"],
        media_types: &["text/x-uuencode"],
        signatures: &[],
        related_formats: &[],
    },
};
