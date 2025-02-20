use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
