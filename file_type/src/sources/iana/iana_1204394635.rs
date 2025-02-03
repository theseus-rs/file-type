use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1204394635: FileFormat = FileFormat {
    id: 1_204_394_635,
    source_type: SourceType::Iana,
    name: "vnd.ms-fontobject",
    extensions: &[],
    media_types: &["application/vnd.ms-fontobject"],
    internal_signatures: &[],
    related_formats: &[],
};
