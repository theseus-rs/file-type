use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_799141244: FileType = FileType {
    file_format: &FileFormat {
        id: 799_141_244,
        source_type: SourceType::Linguist,
        name: "Meson",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
