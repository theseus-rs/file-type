use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_826404698: FileType = FileType {
    file_format: &FileFormat {
        id: 826_404_698,
        source_type: SourceType::Linguist,
        name: "SugarSS",
        extensions: &["sss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
