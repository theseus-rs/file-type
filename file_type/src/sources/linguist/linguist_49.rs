use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_49: FileType = FileType {
    file_format: &FileFormat {
        id: 49,
        source_type: SourceType::Linguist,
        name: "COLLADA",
        extensions: &["dae"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
