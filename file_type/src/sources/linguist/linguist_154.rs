use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_154: FileType = FileType {
    file_format: &FileFormat {
        id: 154,
        source_type: SourceType::Linguist,
        name: "Haml",
        extensions: &["haml", "haml.deface"],
        media_types: &["text/x-haml"],
        signatures: &[],
        related_formats: &[],
    },
};
