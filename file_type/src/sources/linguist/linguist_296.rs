use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_296: FileFormat = FileFormat {
    id: 296,
    source_type: SourceType::Linguist,
    name: "Propeller Spin",
    extensions: &["spin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
