use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_412: FileFormat = FileFormat {
    id: 412,
    source_type: SourceType::Linguist,
    name: "desktop",
    extensions: &["desktop", "desktop.in", "service"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
