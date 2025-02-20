use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_119: FileType = FileType {
    file_format: &FileFormat {
        id: 119,
        source_type: SourceType::Linguist,
        name: "GAP",
        extensions: &["g", "gap", "gd", "gi", "tst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
