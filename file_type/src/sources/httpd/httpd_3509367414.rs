use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3509367414: FileType = FileType {
    file_format: &FileFormat {
        id: 3_509_367_414,
        source_type: SourceType::Httpd,
        name: "mods xml",
        extensions: &["mods"],
        media_types: &["application/mods+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
