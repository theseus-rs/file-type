use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_226: FileType = FileType {
    file_format: &FileFormat {
        id: 226,
        source_type: SourceType::Linguist,
        name: "Maven POM",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
