use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_658971832: FileType = FileType {
    file_format: &FileFormat {
        id: 658_971_832,
        source_type: SourceType::Linguist,
        name: "AL",
        extensions: &["al"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
