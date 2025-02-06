use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_599569029: FileFormat = FileFormat {
    id: 599_569_029,
    source_type: SourceType::Iana,
    name: "vnd.mitsubishi.misty-guard.trustweb",
    extensions: &[],
    media_types: &["application/vnd.mitsubishi.misty-guard.trustweb"],
    signatures: &[],
    related_formats: &[],
};
