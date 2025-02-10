use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_560883276: FileType = FileType {
    file_format: &FileFormat {
        id: 560_883_276,
        source_type: SourceType::Linguist,
        name: "hoon",
        extensions: &["hoon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
