use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3171097430: FileType = FileType {
    file_format: &FileFormat {
        id: 3_171_097_430,
        source_type: SourceType::Iana,
        name: "missing-blocks+cbor-seq",
        extensions: &[],
        media_types: &["application/missing-blocks+cbor-seq"],
        signatures: &[],
        related_formats: &[],
    },
};
