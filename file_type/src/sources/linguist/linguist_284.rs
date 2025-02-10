use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_284: FileType = FileType {
    file_format: &FileFormat {
        id: 284,
        source_type: SourceType::Linguist,
        name: "Pickle",
        extensions: &["pkl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
