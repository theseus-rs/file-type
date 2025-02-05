use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4130112704: FileFormat = FileFormat {
    id: 4_130_112_704,
    source_type: SourceType::Iana,
    name: "cbor-seq",
    extensions: &[],
    media_types: &["application/cbor-seq"],
    signatures: &[],
    related_formats: &[],
};
