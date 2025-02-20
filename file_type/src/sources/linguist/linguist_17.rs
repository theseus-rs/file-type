use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_17: FileType = FileType {
    file_format: &FileFormat {
        id: 17,
        source_type: SourceType::Linguist,
        name: "Apex",
        extensions: &["apex", "cls", "trigger"],
        media_types: &["text/x-java"],
        signatures: &[],
        related_formats: &[],
    },
};
