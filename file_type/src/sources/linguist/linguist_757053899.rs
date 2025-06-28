use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_757053899: FileType = FileType {
    file_format: &FileFormat {
        id: 757_053_899,
        source_type: SourceType::Linguist,
        name: "Vento",
        extensions: &["vto"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
