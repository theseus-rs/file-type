use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
