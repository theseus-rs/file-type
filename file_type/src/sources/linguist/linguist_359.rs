use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_359: FileType = FileType {
    file_format: &FileFormat {
        id: 359,
        source_type: SourceType::Linguist,
        name: "Stylus",
        extensions: &["styl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
