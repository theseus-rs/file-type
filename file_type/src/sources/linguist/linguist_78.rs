use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_78: FileType = FileType {
    file_format: &FileFormat {
        id: 78,
        source_type: SourceType::Linguist,
        name: "Cycript",
        extensions: &["cy"],
        media_types: &["text/javascript"],
        signatures: &[],
        related_formats: &[],
    },
};
