use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
