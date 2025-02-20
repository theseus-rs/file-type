use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3852988807: FileType = FileType {
    file_format: &FileFormat {
        id: 3_852_988_807,
        source_type: SourceType::Iana,
        name: "tve-trigger",
        extensions: &[],
        media_types: &["application/tve-trigger"],
        signatures: &[],
        related_formats: &[],
    },
};
