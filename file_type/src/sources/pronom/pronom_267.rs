use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_267: FileType = FileType {
    file_format: &FileFormat {
        id: 267,
        source_type: SourceType::Pronom,
        name: "IRIS Graphics",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
