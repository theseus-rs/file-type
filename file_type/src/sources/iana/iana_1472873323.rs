use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1472873323: FileFormat = FileFormat {
    id: 1_472_873_323,
    source_type: SourceType::Iana,
    name: "swid+xml",
    extensions: &[],
    media_types: &["application/swid+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
