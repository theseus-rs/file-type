use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_11: FileType = FileType {
    file_format: &FileFormat {
        id: 11,
        source_type: SourceType::Linguist,
        name: "Ada",
        extensions: &["ada", "adb", "ads"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
