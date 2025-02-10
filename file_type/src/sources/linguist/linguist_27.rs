use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_27: FileType = FileType {
    file_format: &FileFormat {
        id: 27,
        source_type: SourceType::Linguist,
        name: "AutoIt",
        extensions: &["au3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
