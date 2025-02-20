use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_218: FileType = FileType {
    file_format: &FileFormat {
        id: 218,
        source_type: SourceType::Linguist,
        name: "MTML",
        extensions: &["mtml"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
