use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_740040417: FileFormat = FileFormat {
    id: 740_040_417,
    source_type: SourceType::Iana,
    name: "vnd.etsi.iptvsad-npvr+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.iptvsad-npvr+xml"],
    signatures: &[],
    related_formats: &[],
};
