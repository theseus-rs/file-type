use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_54: FileFormat = FileFormat {
    id: 54,
    source_type: SourceType::Linguist,
    name: "Ceylon",
    extensions: &["ceylon"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
