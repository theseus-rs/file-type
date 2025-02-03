use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2985206074: FileFormat = FileFormat {
    id: 2_985_206_074,
    source_type: SourceType::Iana,
    name: "octet-stream",
    extensions: &[],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
