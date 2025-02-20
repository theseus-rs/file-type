use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
