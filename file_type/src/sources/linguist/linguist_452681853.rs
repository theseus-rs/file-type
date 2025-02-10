use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_452681853: FileType = FileType {
    file_format: &FileFormat {
        id: 452_681_853,
        source_type: SourceType::Linguist,
        name: "ABAP CDS",
        extensions: &["asddls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
