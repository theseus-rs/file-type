use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_916034822: FileType = FileType {
    file_format: &FileFormat {
        id: 916_034_822,
        source_type: SourceType::Linguist,
        name: "Leo",
        extensions: &["leo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
