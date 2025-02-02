use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_223: FileFormat = FileFormat {
    id: 223,
    source_type: SourceType::Linguist,
    name: "Mask",
    extensions: &["mask"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
