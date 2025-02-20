use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_27: FileType = FileType {
    file_format: &FileFormat {
        id: 27,
        source_type: SourceType::Pronom,
        name: "Revisable-Form-Text Document Content Architecture",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
