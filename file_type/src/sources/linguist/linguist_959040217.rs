use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_959040217: FileType = FileType {
    file_format: &FileFormat {
        id: 959_040_217,
        source_type: SourceType::Linguist,
        name: "Hurl",
        extensions: &["hurl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
