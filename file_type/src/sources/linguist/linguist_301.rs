use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_301: FileFormat = FileFormat {
    id: 301,
    source_type: SourceType::Linguist,
    name: "PureBasic",
    extensions: &["pb", "pbi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
