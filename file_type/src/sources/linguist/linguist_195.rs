use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_195: FileFormat = FileFormat {
    id: 195,
    source_type: SourceType::Linguist,
    name: "Lasso",
    extensions: &["las", "lasso", "lasso8", "lasso9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
