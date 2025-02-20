use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
