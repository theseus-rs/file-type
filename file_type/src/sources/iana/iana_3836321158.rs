use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3836321158: FileFormat = FileFormat {
    id: 3_836_321_158,
    source_type: SourceType::Iana,
    name: "vnd.marlin.drm.mdcf",
    extensions: &[],
    media_types: &["application/vnd.marlin.drm.mdcf"],
    internal_signatures: &[],
    related_formats: &[],
};
