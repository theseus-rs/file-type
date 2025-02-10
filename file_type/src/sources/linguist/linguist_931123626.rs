use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_931123626: FileType = FileType {
    file_format: &FileFormat {
        id: 931_123_626,
        source_type: SourceType::Linguist,
        name: "KDL",
        extensions: &["kdl"],
        media_types: &["text/x-yacas"],
        signatures: &[],
        related_formats: &[],
    },
};
