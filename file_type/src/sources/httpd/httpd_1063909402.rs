use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1063909402: FileType = FileType {
    file_format: &FileFormat {
        id: 1_063_909_402,
        source_type: SourceType::Httpd,
        name: "criticaltools wbs xml",
        extensions: &["wbs"],
        media_types: &["application/vnd.criticaltools.wbs+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
