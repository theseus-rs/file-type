use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_400689417: FileFormat = FileFormat {
    id: 400_689_417,
    source_type: SourceType::Iana,
    name: "csv",
    extensions: &[],
    media_types: &["text/csv"],
    signatures: &[],
    related_formats: &[],
};
