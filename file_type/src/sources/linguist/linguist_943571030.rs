use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_943571030: FileFormat = FileFormat {
    id: 943_571_030,
    source_type: SourceType::Linguist,
    name: "BrighterScript",
    extensions: &["bs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
