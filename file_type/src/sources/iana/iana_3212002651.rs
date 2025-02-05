use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3212002651: FileFormat = FileFormat {
    id: 3_212_002_651,
    source_type: SourceType::Iana,
    name: "vnd.yaoweme",
    extensions: &[],
    media_types: &["application/vnd.yaoweme"],
    signatures: &[],
    related_formats: &[],
};
