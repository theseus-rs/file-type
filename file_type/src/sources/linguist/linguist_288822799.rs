use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_288822799: FileFormat = FileFormat {
    id: 288_822_799,
    source_type: SourceType::Linguist,
    name: "Pkl",
    extensions: &["pkl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
