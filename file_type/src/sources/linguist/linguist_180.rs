use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_180: FileFormat = FileFormat {
    id: 180,
    source_type: SourceType::Linguist,
    name: "Jasmin",
    extensions: &["j"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
