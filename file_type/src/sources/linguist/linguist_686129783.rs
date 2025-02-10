use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_686129783: FileType = FileType {
    file_format: &FileFormat {
        id: 686_129_783,
        source_type: SourceType::Linguist,
        name: "FIGlet Font",
        extensions: &["flf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
