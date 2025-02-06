use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_546384924: FileFormat = FileFormat {
    id: 546_384_924,
    source_type: SourceType::Iana,
    name: "vnd.medicalholodeck.recordxr",
    extensions: &[],
    media_types: &["application/vnd.medicalholodeck.recordxr"],
    signatures: &[],
    related_formats: &[],
};
