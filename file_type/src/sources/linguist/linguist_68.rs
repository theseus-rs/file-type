use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_68: FileType = FileType {
    file_format: &FileFormat {
        id: 68,
        source_type: SourceType::Linguist,
        name: "Cool",
        extensions: &["cl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
