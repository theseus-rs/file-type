use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_970539067: FileType = FileType {
    file_format: &FileFormat {
        id: 970_539_067,
        source_type: SourceType::Linguist,
        name: "q",
        extensions: &["q"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
