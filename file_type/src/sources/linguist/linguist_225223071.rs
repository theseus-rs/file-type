use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_225223071: FileType = FileType {
    file_format: &FileFormat {
        id: 225_223_071,
        source_type: SourceType::Linguist,
        name: "Xmake",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
