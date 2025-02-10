use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
