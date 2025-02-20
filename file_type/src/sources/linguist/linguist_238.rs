use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_238: FileType = FileType {
    file_format: &FileFormat {
        id: 238,
        source_type: SourceType::Linguist,
        name: "MoonScript",
        extensions: &["moon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
