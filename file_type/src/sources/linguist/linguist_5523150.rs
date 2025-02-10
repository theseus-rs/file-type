use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_5523150: FileType = FileType {
    file_format: &FileFormat {
        id: 5_523_150,
        source_type: SourceType::Linguist,
        name: "Glimmer JS",
        extensions: &["gjs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
