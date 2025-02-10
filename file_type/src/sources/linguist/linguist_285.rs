use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_285: FileType = FileType {
    file_format: &FileFormat {
        id: 285,
        source_type: SourceType::Linguist,
        name: "PicoLisp",
        extensions: &["l"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
