use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_15793840: FileFormat = FileFormat {
    id: 15_793_840,
    source_type: SourceType::Iana,
    name: "vnd.nervana",
    extensions: &[],
    media_types: &["application/vnd.nervana"],
    internal_signatures: &[],
    related_formats: &[],
};
