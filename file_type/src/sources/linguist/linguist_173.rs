use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_173: FileType = FileType {
    file_format: &FileFormat {
        id: 173,
        source_type: SourceType::Linguist,
        name: "JFlex",
        extensions: &["flex", "jflex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
