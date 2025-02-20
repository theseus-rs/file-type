use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12236515: FileType = FileType {
    file_format: &FileFormat {
        id: 12_236_515,
        source_type: SourceType::Httpd,
        name: "pict",
        extensions: &["pic", "pct"],
        media_types: &["image/x-pict"],
        signatures: &[],
        related_formats: &[],
    },
};
