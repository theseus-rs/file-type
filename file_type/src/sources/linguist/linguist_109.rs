use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_109: FileFormat = FileFormat {
    id: 109,
    source_type: SourceType::Linguist,
    name: "Fancy",
    extensions: &["fancypack", "fy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
