use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1298869663: FileFormat = FileFormat {
    id: 1_298_869_663,
    source_type: SourceType::Iana,
    name: "vnd.think-cell.ppttc+json",
    extensions: &[],
    media_types: &["application/vnd.think-cell.ppttc+json"],
    internal_signatures: &[],
    related_formats: &[],
};
