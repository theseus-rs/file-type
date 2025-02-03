use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2592267841: FileFormat = FileFormat {
    id: 2_592_267_841,
    source_type: SourceType::Iana,
    name: "sarif-external-properties+json",
    extensions: &[],
    media_types: &["application/sarif-external-properties+json"],
    internal_signatures: &[],
    related_formats: &[],
};
