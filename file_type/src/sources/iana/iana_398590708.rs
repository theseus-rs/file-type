use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_398590708: FileFormat = FileFormat {
    id: 398_590_708,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.drm-trigger+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.drm-trigger+xml"],
    signatures: &[],
    related_formats: &[],
};
