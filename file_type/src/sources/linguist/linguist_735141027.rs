use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_735141027: FileType = FileType {
    file_format: &FileFormat {
        id: 735_141_027,
        source_type: SourceType::Linguist,
        name: "SurrealQL",
        extensions: &["surql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
