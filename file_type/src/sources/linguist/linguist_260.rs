use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_260: FileFormat = FileFormat {
    id: 260,
    source_type: SourceType::Linguist,
    name: "Omgrofl",
    extensions: &["omgrofl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
