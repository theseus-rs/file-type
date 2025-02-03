use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_988896440: FileFormat = FileFormat {
    id: 988_896_440,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.seal-ue-config-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.seal-ue-config-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
