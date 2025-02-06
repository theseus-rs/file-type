use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3274693461: FileFormat = FileFormat {
    id: 3_274_693_461,
    source_type: SourceType::Iana,
    name: "nss",
    extensions: &[],
    media_types: &["application/nss"],
    signatures: &[],
    related_formats: &[],
};
