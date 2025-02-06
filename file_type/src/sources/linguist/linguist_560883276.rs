use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_560883276: FileFormat = FileFormat {
    id: 560_883_276,
    source_type: SourceType::Linguist,
    name: "hoon",
    extensions: &["hoon"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
