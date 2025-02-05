use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_973483626: FileFormat = FileFormat {
    id: 973_483_626,
    source_type: SourceType::Linguist,
    name: "ZIL",
    extensions: &["mud", "zil"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
