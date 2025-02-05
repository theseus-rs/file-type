use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3987383493: FileFormat = FileFormat {
    id: 3_987_383_493,
    source_type: SourceType::Iana,
    name: "jxr",
    extensions: &[],
    media_types: &["image/jxr"],
    signatures: &[],
    related_formats: &[],
};
