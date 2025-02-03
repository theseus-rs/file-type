use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3514429168: FileFormat = FileFormat {
    id: 3_514_429_168,
    source_type: SourceType::Iana,
    name: "vnd.street-stream",
    extensions: &[],
    media_types: &["application/vnd.street-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
