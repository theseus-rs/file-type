use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_290345951: FileFormat = FileFormat {
    id: 290_345_951,
    source_type: SourceType::Linguist,
    name: "GAML",
    extensions: &["gaml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
