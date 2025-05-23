use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
