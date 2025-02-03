use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1370014260: FileFormat = FileFormat {
    id: 1_370_014_260,
    source_type: SourceType::Iana,
    name: "vnd.oma.cab-feature-handler+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.cab-feature-handler+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
