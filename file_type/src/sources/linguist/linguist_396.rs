use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_396: FileType = FileType {
    file_format: &FileFormat {
        id: 396,
        source_type: SourceType::Linguist,
        name: "World of Warcraft Addon Data",
        extensions: &["toc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
