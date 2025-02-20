use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_200: FileType = FileType {
    file_format: &FileFormat {
        id: 200,
        source_type: SourceType::Linguist,
        name: "LilyPond",
        extensions: &["ily", "ly"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
