use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_209: FileFormat = FileFormat {
    id: 209,
    source_type: SourceType::Linguist,
    name: "Logos",
    extensions: &["x", "xi", "xm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
