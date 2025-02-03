use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_423816041: FileFormat = FileFormat {
    id: 423_816_041,
    source_type: SourceType::Iana,
    name: "wspolicy+xml",
    extensions: &[],
    media_types: &["application/wspolicy+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
