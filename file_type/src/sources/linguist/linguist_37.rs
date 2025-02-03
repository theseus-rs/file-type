use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_37: FileFormat = FileFormat {
    id: 37,
    source_type: SourceType::Linguist,
    name: "Boo",
    extensions: &["boo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
