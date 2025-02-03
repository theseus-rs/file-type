use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_356: FileFormat = FileFormat {
    id: 356,
    source_type: SourceType::Linguist,
    name: "Stan",
    extensions: &["stan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
