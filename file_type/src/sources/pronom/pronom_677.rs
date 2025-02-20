use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_677: FileType = FileType {
    file_format: &FileFormat {
        id: 677,
        source_type: SourceType::Pronom,
        name: "Still Picture Interchange File Format",
        extensions: &[],
        media_types: &["image/jpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
