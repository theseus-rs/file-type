use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_111148035: FileType = FileType {
    file_format: &FileFormat {
        id: 111_148_035,
        source_type: SourceType::Linguist,
        name: "Dotenv",
        extensions: &["env"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
