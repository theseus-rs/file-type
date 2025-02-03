use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_11: FileFormat = FileFormat {
    id: 11,
    source_type: SourceType::Linguist,
    name: "Ada",
    extensions: &["ada", "adb", "ads"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
