use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_290: FileFormat = FileFormat {
    id: 290,
    source_type: SourceType::Linguist,
    name: "Pony",
    extensions: &["pony"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
