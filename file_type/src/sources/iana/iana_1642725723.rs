use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1642725723: FileFormat = FileFormat {
    id: 1_642_725_723,
    source_type: SourceType::Iana,
    name: "CEA",
    extensions: &[],
    media_types: &["application/CEA"],
    internal_signatures: &[],
    related_formats: &[],
};
