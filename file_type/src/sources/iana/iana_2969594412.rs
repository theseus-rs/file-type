use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2969594412: FileType = FileType {
    file_format: &FileFormat {
        id: 2_969_594_412,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-network-resource-info+cbor",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-network-resource-info+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
