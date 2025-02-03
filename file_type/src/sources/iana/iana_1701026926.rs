use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1701026926: FileFormat = FileFormat {
    id: 1_701_026_926,
    source_type: SourceType::Iana,
    name: "clr",
    extensions: &[],
    media_types: &["application/clr"],
    internal_signatures: &[],
    related_formats: &[],
};
