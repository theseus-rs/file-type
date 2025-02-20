use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_109: FileType = FileType {
    file_format: &FileFormat {
        id: 109,
        source_type: SourceType::Linguist,
        name: "Fancy",
        extensions: &["fancypack", "fy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
