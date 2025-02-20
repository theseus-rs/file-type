use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1020148948: FileType = FileType {
    file_format: &FileFormat {
        id: 1_020_148_948,
        source_type: SourceType::Linguist,
        name: "Redirect Rules",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
