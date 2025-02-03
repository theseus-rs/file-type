use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3151184436: FileFormat = FileFormat {
    id: 3_151_184_436,
    source_type: SourceType::Iana,
    name: "vnd.marlin.drm.conftoken+xml",
    extensions: &[],
    media_types: &["application/vnd.marlin.drm.conftoken+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
