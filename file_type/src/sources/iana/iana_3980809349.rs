use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3980809349: FileFormat = FileFormat {
    id: 3_980_809_349,
    source_type: SourceType::Iana,
    name: "vnd.nebumind.line",
    extensions: &[],
    media_types: &["application/vnd.nebumind.line"],
    signatures: &[],
    related_formats: &[],
};
