use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_362: FileType = FileType {
    file_format: &FileFormat {
        id: 362,
        source_type: SourceType::Pronom,
        name: "Microsoft Project",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
