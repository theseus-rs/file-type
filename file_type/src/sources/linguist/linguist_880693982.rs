use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_880693982: FileType = FileType {
    file_format: &FileFormat {
        id: 880_693_982,
        source_type: SourceType::Linguist,
        name: "Euphoria",
        extensions: &["e", "ex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
