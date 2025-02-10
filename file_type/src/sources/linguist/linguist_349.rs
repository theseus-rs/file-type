use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_349: FileType = FileType {
    file_format: &FileFormat {
        id: 349,
        source_type: SourceType::Linguist,
        name: "Slash",
        extensions: &["sl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
