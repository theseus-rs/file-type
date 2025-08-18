use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_809230569: FileType = FileType {
    file_format: &FileFormat {
        id: 809_230_569,
        source_type: SourceType::Linguist,
        name: "ROS Interface",
        extensions: &["action", "msg", "srv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
