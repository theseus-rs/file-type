use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1936061579: FileFormat = FileFormat {
    id: 1_936_061_579,
    source_type: SourceType::Iana,
    name: "vnd.marlin.drm.actiontoken+xml",
    extensions: &[],
    media_types: &["application/vnd.marlin.drm.actiontoken+xml"],
    signatures: &[],
    related_formats: &[],
};
