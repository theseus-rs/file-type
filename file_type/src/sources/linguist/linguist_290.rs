use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_290: FileType = FileType {
    file_format: &FileFormat {
        id: 290,
        source_type: SourceType::Linguist,
        name: "Pony",
        extensions: &["pony"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
