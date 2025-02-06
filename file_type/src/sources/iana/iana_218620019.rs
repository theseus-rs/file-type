use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_218620019: FileFormat = FileFormat {
    id: 218_620_019,
    source_type: SourceType::Iana,
    name: "vnd.dolby.mobile.1",
    extensions: &[],
    media_types: &["application/vnd.dolby.mobile.1"],
    signatures: &[],
    related_formats: &[],
};
