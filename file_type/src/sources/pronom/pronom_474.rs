use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_474: FileType = FileType {
    file_format: &FileFormat {
        id: 474,
        source_type: SourceType::Pronom,
        name: "Document Type Definition",
        extensions: &["dtd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
