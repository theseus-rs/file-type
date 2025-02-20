use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_101: FileType = FileType {
    file_format: &FileFormat {
        id: 101,
        source_type: SourceType::Linguist,
        name: "Elm",
        extensions: &["elm"],
        media_types: &["text/x-elm"],
        signatures: &[],
        related_formats: &[],
    },
};
