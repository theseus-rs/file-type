use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_106: FileType = FileType {
    file_format: &FileFormat {
        id: 106,
        source_type: SourceType::Linguist,
        name: "FLUX",
        extensions: &["flux", "fx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
