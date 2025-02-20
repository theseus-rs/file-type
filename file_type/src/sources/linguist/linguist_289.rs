use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
