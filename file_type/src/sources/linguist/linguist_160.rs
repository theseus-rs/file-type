use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_160: FileFormat = FileFormat {
    id: 160,
    source_type: SourceType::Linguist,
    name: "HyPhy",
    extensions: &["bf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
