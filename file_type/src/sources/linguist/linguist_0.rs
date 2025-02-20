use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_0: FileType = FileType {
    file_format: &FileFormat {
        id: 0,
        source_type: SourceType::Linguist,
        name: "1C Enterprise",
        extensions: &["bsl", "os"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
