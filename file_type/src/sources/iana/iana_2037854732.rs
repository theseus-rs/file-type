use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2037854732: FileFormat = FileFormat {
    id: 2_037_854_732,
    source_type: SourceType::Iana,
    name: "prs.xsf+xml",
    extensions: &[],
    media_types: &["application/prs.xsf+xml"],
    signatures: &[],
    related_formats: &[],
};
