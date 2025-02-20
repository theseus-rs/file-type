use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_260: FileType = FileType {
    file_format: &FileFormat {
        id: 260,
        source_type: SourceType::Linguist,
        name: "Omgrofl",
        extensions: &["omgrofl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
