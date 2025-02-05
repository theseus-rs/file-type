use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1283070147: FileFormat = FileFormat {
    id: 1_283_070_147,
    source_type: SourceType::Iana,
    name: "vnd.xmpie.ppkg",
    extensions: &[],
    media_types: &["application/vnd.xmpie.ppkg"],
    signatures: &[],
    related_formats: &[],
};
