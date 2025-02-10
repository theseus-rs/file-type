use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2150: FileType = FileType {
    file_format: &FileFormat {
        id: 2_150,
        source_type: SourceType::Pronom,
        name: "HP Photo Album",
        extensions: &["albm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
