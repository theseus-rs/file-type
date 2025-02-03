use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_404372107: FileFormat = FileFormat {
    id: 404_372_107,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcdata-ue-config+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcdata-ue-config+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
