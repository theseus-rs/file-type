use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_321: FileType = FileType {
    file_format: &FileFormat {
        id: 321,
        source_type: SourceType::Linguist,
        name: "Redcode",
        extensions: &["cw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
