use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_289: FileType = FileType {
    file_format: &FileFormat {
        id: 289,
        source_type: SourceType::Linguist,
        name: "PogoScript",
        extensions: &["pogo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
