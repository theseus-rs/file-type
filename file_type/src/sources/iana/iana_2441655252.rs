use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2441655252: FileFormat = FileFormat {
    id: 2_441_655_252,
    source_type: SourceType::Iana,
    name: "vnd.oma.drm.risd+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.drm.risd+xml"],
    signatures: &[],
    related_formats: &[],
};
