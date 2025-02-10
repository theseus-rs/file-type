use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_448253929: FileType = FileType {
    file_format: &FileFormat {
        id: 448_253_929,
        source_type: SourceType::Linguist,
        name: "MLIR",
        extensions: &["mlir"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
