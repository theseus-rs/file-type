use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2280676910: FileFormat = FileFormat {
    id: 2_280_676_910,
    source_type: SourceType::Iana,
    name: "vnd.publishare-delta-tree",
    extensions: &[],
    media_types: &["application/vnd.publishare-delta-tree"],
    internal_signatures: &[],
    related_formats: &[],
};
