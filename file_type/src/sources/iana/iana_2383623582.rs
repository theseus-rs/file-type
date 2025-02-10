use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2383623582: FileType = FileType {
    file_format: &FileFormat {
        id: 2_383_623_582,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.themeOverride+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.themeOverride+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
