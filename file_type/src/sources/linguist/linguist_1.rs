use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1: FileFormat = FileFormat {
    id: 1,
    source_type: SourceType::Linguist,
    name: "ABAP",
    extensions: &["abap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
