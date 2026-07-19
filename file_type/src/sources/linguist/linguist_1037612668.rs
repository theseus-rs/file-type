use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1037612668: FileType = FileType {
    file_format: &FileFormat {
        id: 1_037_612_668,
        source_type: SourceType::Linguist,
        name: "MeTTa",
        extensions: &["metta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
