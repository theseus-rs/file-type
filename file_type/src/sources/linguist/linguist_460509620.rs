use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_460509620: FileFormat = FileFormat {
    id: 460_509_620,
    source_type: SourceType::Linguist,
    name: "Edge",
    extensions: &["edge"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
