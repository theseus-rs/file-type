use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_794680800: FileType = FileType {
    file_format: &FileFormat {
        id: 794_680_800,
        source_type: SourceType::Httpd,
        name: "applixware",
        extensions: &["aw"],
        media_types: &["application/applixware"],
        signatures: &[],
        related_formats: &[],
    },
};
