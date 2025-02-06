use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_116: FileFormat = FileFormat {
    id: 116,
    source_type: SourceType::Linguist,
    name: "Frege",
    extensions: &["fr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
