use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_98: FileFormat = FileFormat {
    id: 98,
    source_type: SourceType::Linguist,
    name: "Ecere Projects",
    extensions: &["epj"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
