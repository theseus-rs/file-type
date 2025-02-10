use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_5: FileType = FileType {
    file_format: &FileFormat {
        id: 5,
        source_type: SourceType::Linguist,
        name: "API Blueprint",
        extensions: &["apib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
