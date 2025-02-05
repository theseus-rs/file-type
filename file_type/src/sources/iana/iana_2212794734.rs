use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2212794734: FileFormat = FileFormat {
    id: 2_212_794_734,
    source_type: SourceType::Iana,
    name: "vnd.rig.cryptonote",
    extensions: &[],
    media_types: &["application/vnd.rig.cryptonote"],
    signatures: &[],
    related_formats: &[],
};
