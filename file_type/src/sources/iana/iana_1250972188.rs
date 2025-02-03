use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1250972188: FileFormat = FileFormat {
    id: 1_250_972_188,
    source_type: SourceType::Iana,
    name: "scvp-vp-response",
    extensions: &[],
    media_types: &["application/scvp-vp-response"],
    internal_signatures: &[],
    related_formats: &[],
};
