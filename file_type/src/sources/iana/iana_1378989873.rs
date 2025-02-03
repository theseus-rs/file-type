use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1378989873: FileFormat = FileFormat {
    id: 1_378_989_873,
    source_type: SourceType::Iana,
    name: "pvd+json",
    extensions: &[],
    media_types: &["application/pvd+json"],
    internal_signatures: &[],
    related_formats: &[],
};
