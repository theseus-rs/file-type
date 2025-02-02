use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_424510560: FileFormat = FileFormat {
    id: 424_510_560,
    source_type: SourceType::Linguist,
    name: "STAR",
    extensions: &["star"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
