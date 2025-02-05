use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4200633700: FileFormat = FileFormat {
    id: 4_200_633_700,
    source_type: SourceType::Iana,
    name: "cid-edhoc+cbor-seq",
    extensions: &[],
    media_types: &["application/cid-edhoc+cbor-seq"],
    signatures: &[],
    related_formats: &[],
};
