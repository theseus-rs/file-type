use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_168: FileType = FileType {
    file_format: &FileFormat {
        id: 168,
        source_type: SourceType::Linguist,
        name: "Io",
        extensions: &["io"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
