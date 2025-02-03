use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_344: FileFormat = FileFormat {
    id: 344,
    source_type: SourceType::Linguist,
    name: "Scilab",
    extensions: &["sce", "sci", "tst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
