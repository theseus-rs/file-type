use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_175: FileType = FileType {
    file_format: &FileFormat {
        id: 175,
        source_type: SourceType::Linguist,
        name: "JSON5",
        extensions: &["json5"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
