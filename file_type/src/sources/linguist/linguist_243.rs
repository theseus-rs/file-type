use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_243: FileType = FileType {
    file_format: &FileFormat {
        id: 243,
        source_type: SourceType::Linguist,
        name: "Nemerle",
        extensions: &["n"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
