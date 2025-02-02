use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_321200902: FileFormat = FileFormat {
    id: 321_200_902,
    source_type: SourceType::Linguist,
    name: "Bicep",
    extensions: &["bicep", "bicepparam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
