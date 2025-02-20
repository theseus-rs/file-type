use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_839112914: FileType = FileType {
    file_format: &FileFormat {
        id: 839_112_914,
        source_type: SourceType::Linguist,
        name: "Polar",
        extensions: &["polar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
