use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_685022663: FileType = FileType {
    file_format: &FileFormat {
        id: 685_022_663,
        source_type: SourceType::Linguist,
        name: "NPM Config",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
