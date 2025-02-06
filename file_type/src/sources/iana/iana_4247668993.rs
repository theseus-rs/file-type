use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4247668993: FileFormat = FileFormat {
    id: 4_247_668_993,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.seal-group-doc+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.seal-group-doc+xml"],
    signatures: &[],
    related_formats: &[],
};
