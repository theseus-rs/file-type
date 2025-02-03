use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_259: FileFormat = FileFormat {
    id: 259,
    source_type: SourceType::Linguist,
    name: "Objective-J",
    extensions: &["j", "sj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
