use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2212252381: FileFormat = FileFormat {
    id: 2_212_252_381,
    source_type: SourceType::Iana,
    name: "vnd.etsi.iptvsad-cod+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.iptvsad-cod+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
