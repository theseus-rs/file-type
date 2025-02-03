use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_441858312: FileFormat = FileFormat {
    id: 441_858_312,
    source_type: SourceType::Linguist,
    name: "Promela",
    extensions: &["pml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
