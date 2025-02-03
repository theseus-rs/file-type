use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2468360867: FileFormat = FileFormat {
    id: 2_468_360_867,
    source_type: SourceType::Iana,
    name: "td+json",
    extensions: &[],
    media_types: &["application/td+json"],
    internal_signatures: &[],
    related_formats: &[],
};
