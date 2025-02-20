use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_521: FileType = FileType {
    file_format: &FileFormat {
        id: 521,
        source_type: SourceType::Pronom,
        name: "SAS Data File",
        extensions: &["ssd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
