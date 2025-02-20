use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_39: FileType = FileType {
    file_format: &FileFormat {
        id: 39,
        source_type: SourceType::Linguist,
        name: "Brightscript",
        extensions: &["brs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
