use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_817442074: FileFormat = FileFormat {
    id: 817_442_074,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.seal-network-QoS-management-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.seal-network-QoS-management-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
