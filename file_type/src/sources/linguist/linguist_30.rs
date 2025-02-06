use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_30: FileFormat = FileFormat {
    id: 30,
    source_type: SourceType::Linguist,
    name: "Befunge",
    extensions: &["befunge", "bf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
