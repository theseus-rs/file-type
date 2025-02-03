use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_173: FileFormat = FileFormat {
    id: 173,
    source_type: SourceType::Linguist,
    name: "JFlex",
    extensions: &["flex", "jflex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
