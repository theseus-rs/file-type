use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_339615665: FileType = FileType {
    file_format: &FileFormat {
        id: 339_615_665,
        source_type: SourceType::Iana,
        name: "vnd.as207960.vas.tap+jer",
        extensions: &[],
        media_types: &["application/vnd.as207960.vas.tap+jer"],
        signatures: &[],
        related_formats: &[],
    },
};
