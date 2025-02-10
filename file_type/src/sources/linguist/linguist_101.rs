use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
