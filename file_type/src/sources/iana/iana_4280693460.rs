use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4280693460: FileFormat = FileFormat {
    id: 4_280_693_460,
    source_type: SourceType::Iana,
    name: "pem-certificate-chain",
    extensions: &[],
    media_types: &["application/pem-certificate-chain"],
    internal_signatures: &[],
    related_formats: &[],
};
