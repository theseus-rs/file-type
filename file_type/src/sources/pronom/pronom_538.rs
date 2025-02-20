use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_538: FileType = FileType {
    file_format: &FileFormat {
        id: 538,
        source_type: SourceType::Pronom,
        name: "VisiCalc Database",
        extensions: &["dif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
