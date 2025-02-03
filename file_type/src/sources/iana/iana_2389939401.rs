use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2389939401: FileFormat = FileFormat {
    id: 2_389_939_401,
    source_type: SourceType::Iana,
    name: "vnd.ms-powerpoint.presentation.macroEnabled.12",
    extensions: &[],
    media_types: &["application/vnd.ms-powerpoint.presentation.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[],
};
