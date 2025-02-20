use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_360: FileType = FileType {
    file_format: &FileFormat {
        id: 360,
        source_type: SourceType::Linguist,
        name: "SubRip Text",
        extensions: &["srt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
