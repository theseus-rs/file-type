use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_880693982: FileFormat = FileFormat {
    id: 880_693_982,
    source_type: SourceType::Linguist,
    name: "Euphoria",
    extensions: &["e", "ex"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
