use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_37: FileType = FileType {
    file_format: &FileFormat {
        id: 37,
        source_type: SourceType::Linguist,
        name: "Boo",
        extensions: &["boo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
