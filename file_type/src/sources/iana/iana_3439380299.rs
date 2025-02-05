use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3439380299: FileFormat = FileFormat {
    id: 3_439_380_299,
    source_type: SourceType::Iana,
    name: "vnd.apache.arrow.stream",
    extensions: &[],
    media_types: &["application/vnd.apache.arrow.stream"],
    signatures: &[],
    related_formats: &[],
};
