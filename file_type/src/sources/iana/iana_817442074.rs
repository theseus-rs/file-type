use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_817442074: FileType = FileType {
    file_format: &FileFormat {
        id: 817_442_074,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-network-QoS-management-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-network-QoS-management-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
