use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_309: FileType = FileType {
    file_format: &FileFormat {
        id: 309,
        source_type: SourceType::Linguist,
        name: "RDoc",
        extensions: &["rdoc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
