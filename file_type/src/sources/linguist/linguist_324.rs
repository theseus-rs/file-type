use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_324: FileType = FileType {
    file_format: &FileFormat {
        id: 324,
        source_type: SourceType::Linguist,
        name: "RobotFramework",
        extensions: &["resource", "robot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
