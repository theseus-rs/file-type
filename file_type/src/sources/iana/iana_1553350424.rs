use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1553350424: FileFormat = FileFormat {
    id: 1_553_350_424,
    source_type: SourceType::Iana,
    name: "vnd.apache.parquet",
    extensions: &[],
    media_types: &["application/vnd.apache.parquet"],
    internal_signatures: &[],
    related_formats: &[],
};
