use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_324: FileFormat = FileFormat {
    id: 324,
    source_type: SourceType::Linguist,
    name: "RobotFramework",
    extensions: &["resource", "robot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
