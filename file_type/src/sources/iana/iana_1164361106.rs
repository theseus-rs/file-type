use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1164361106: FileFormat = FileFormat {
    id: 1_164_361_106,
    source_type: SourceType::Iana,
    name: "fits",
    extensions: &[],
    media_types: &["application/fits"],
    internal_signatures: &[],
    related_formats: &[],
};
