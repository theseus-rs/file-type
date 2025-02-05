use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_665519283: FileFormat = FileFormat {
    id: 665_519_283,
    source_type: SourceType::Iana,
    name: "vnd.afpc.foca-codedfont",
    extensions: &[],
    media_types: &["application/vnd.afpc.foca-codedfont"],
    signatures: &[],
    related_formats: &[],
};
