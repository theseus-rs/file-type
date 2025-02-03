use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_552814135: FileFormat = FileFormat {
    id: 552_814_135,
    source_type: SourceType::Iana,
    name: "vnd.dataresource+json",
    extensions: &[],
    media_types: &["application/vnd.dataresource+json"],
    internal_signatures: &[],
    related_formats: &[],
};
