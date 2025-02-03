use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1347231439: FileFormat = FileFormat {
    id: 1_347_231_439,
    source_type: SourceType::Iana,
    name: "vnd.marlin.drm.license+xml",
    extensions: &[],
    media_types: &["application/vnd.marlin.drm.license+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
