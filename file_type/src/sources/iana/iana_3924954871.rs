use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3924954871: FileFormat = FileFormat {
    id: 3_924_954_871,
    source_type: SourceType::Iana,
    name: "prs.nprend",
    extensions: &[],
    media_types: &["application/prs.nprend"],
    signatures: &[],
    related_formats: &[],
};
