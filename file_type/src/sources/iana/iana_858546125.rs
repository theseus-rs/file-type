use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_858546125: FileFormat = FileFormat {
    id: 858_546_125,
    source_type: SourceType::Iana,
    name: "vnd.blink-idb-value-wrapper",
    extensions: &[],
    media_types: &["application/vnd.blink-idb-value-wrapper"],
    signatures: &[],
    related_formats: &[],
};
