use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2705: FileType = FileType {
    file_format: &FileFormat {
        id: 2_705,
        source_type: SourceType::Pronom,
        name: "Camtasia Recording File",
        extensions: &["camrec"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
