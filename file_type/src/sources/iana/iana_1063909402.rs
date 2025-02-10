use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1063909402: FileType = FileType {
    file_format: &FileFormat {
        id: 1_063_909_402,
        source_type: SourceType::Iana,
        name: "vnd.criticaltools.wbs+xml",
        extensions: &[],
        media_types: &["application/vnd.criticaltools.wbs+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
