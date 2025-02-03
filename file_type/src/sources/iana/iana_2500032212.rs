use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2500032212: FileFormat = FileFormat {
    id: 2_500_032_212,
    source_type: SourceType::Iana,
    name: "vnd.geocube+xml (OBSOLETED by request)",
    extensions: &[],
    media_types: &["application/vnd.geocube+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
