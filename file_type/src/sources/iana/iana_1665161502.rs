use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1665161502: FileFormat = FileFormat {
    id: 1_665_161_502,
    source_type: SourceType::Iana,
    name: "dash+xml",
    extensions: &[],
    media_types: &["application/dash+xml"],
    signatures: &[],
    related_formats: &[],
};
