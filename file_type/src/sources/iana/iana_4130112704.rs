use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4130112704: FileType = FileType {
    file_format: &FileFormat {
        id: 4_130_112_704,
        source_type: SourceType::Iana,
        name: "cbor-seq",
        extensions: &[],
        media_types: &["application/cbor-seq"],
        signatures: &[],
        related_formats: &[],
    },
};
