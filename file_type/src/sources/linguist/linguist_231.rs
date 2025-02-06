use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_231: FileFormat = FileFormat {
    id: 231,
    source_type: SourceType::Linguist,
    name: "MiniD",
    extensions: &["minid"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
