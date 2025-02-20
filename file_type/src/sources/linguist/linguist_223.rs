use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_223: FileType = FileType {
    file_format: &FileFormat {
        id: 223,
        source_type: SourceType::Linguist,
        name: "Mask",
        extensions: &["mask"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
