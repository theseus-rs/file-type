use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_717604115: FileFormat = FileFormat {
    id: 717_604_115,
    source_type: SourceType::Iana,
    name: "vnd.oai.workflows+json",
    extensions: &[],
    media_types: &["application/vnd.oai.workflows+json"],
    signatures: &[],
    related_formats: &[],
};
