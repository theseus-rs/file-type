use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_364: FileFormat = FileFormat {
    id: 364,
    source_type: SourceType::Linguist,
    name: "TLA",
    extensions: &["tla"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
