use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_13: FileFormat = FileFormat {
    id: 13,
    source_type: SourceType::Linguist,
    name: "Alloy",
    extensions: &["als"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
