use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_284: FileFormat = FileFormat {
    id: 284,
    source_type: SourceType::Linguist,
    name: "Pickle",
    extensions: &["pkl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
