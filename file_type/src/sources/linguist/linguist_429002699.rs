use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_429002699: FileType = FileType {
    file_format: &FileFormat {
        id: 429_002_699,
        source_type: SourceType::Linguist,
        name: "Mathematical Programming System",
        extensions: &["mps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
